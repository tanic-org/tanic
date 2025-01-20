use crate::iceberg_context::IcebergContext;
use http::Uri;
use iceberg::NamespaceIdent;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use tanic_core::config::ConnectionDetails;
use tanic_core::message::{NamespaceDeets, TableDeets, TanicMessage};
use tanic_core::Result;
use tanic_core::TanicConfig;
use tokio::sync::mpsc::{Receiver as MpscReceiver, Sender as MpscSender};
use tokio::sync::watch::{Receiver as WatchReceiver, Sender as WatchSender};

pub mod iceberg_context;

#[derive(Debug)]
pub enum TanicAction {
    Exit,

    ConnectTo(ConnectionDetails),

    FocusPrevNamespace,
    FocusNextNamespace,
    SelectNamespace,

    FocusPrevTable,
    FocusNextTable,
    SelectTable,
}

#[derive(Clone, Debug)]
pub enum TanicAppState {
    Initializing,
    ViewingNamespacesList(ViewingNamespacesListState),
    ViewingTablesList(ViewingTablesListState),
    Exiting,
}

#[derive(Clone, Debug)]
pub struct ViewingNamespacesListState {
    namespaces: Vec<NamespaceDeets>,
    selected_idx: usize,
}

#[derive(Clone, Debug)]
pub struct ViewingTablesListState {
    pub namespace: String,
    pub tables: Vec<TableDeets>,
    pub selected_idx: usize,
}

// #[derive(Clone, Debug)]
// pub struct TanicAppState {
//     should_exit: bool,
//     config: TanicConfig,
//
//     open_connections: Vec<IcebergContext>,
// }

pub struct AppStateManager {
    action_rx: MpscReceiver<TanicAction>,
    action_tx: MpscSender<TanicAction>,
    state_tx: WatchSender<TanicAppState>,

    state: TanicAppState,
}

impl AppStateManager {
    pub fn new(
        config: TanicConfig
    ) -> (Self, MpscSender<TanicAction>, WatchReceiver<TanicAppState>) {
        let state = TanicAppState::default()
            .with_config(config);
        let (action_tx, action_rx) = tokio::sync::mpsc::unbounded_channel();
        let (state_tx, state_rx) = tokio::sync::watch::channel(state.clone());

        (Self {
            action_rx,
            action_tx: action_tx.clone(),
            state_tx,
            state
        }, action_tx, state_rx)
    }

    async fn event_loop(mut self) -> Result<()> {
        let Self {
            mut state, mut state_tx, mut action_rx, mut action_tx
        } = self;

        while !matches!(state, TanicAppState::Exiting) {
            let Some(action) = action_rx.recv().await else {
                break;
            };
            tracing::info!(?action, "AppState received an action");

            let (next_state, action) = match action {
                TanicAction::Exit => {
                    (
                        state.should_exit(), None
                    )
                }

                TanicAction::ConnectTo(connection_details) => {
                    self.connect_to(connection_details).await?;
                }

                _ => {}
            };

            state = next_state;
            state_tx.send(state.clone()).await;
            if let Some(action) = action {
                let _ = action_tx.send(action).await?;
            }
        }

        Ok(())
    }

    async fn connect_to(&mut self, conn_details: ConnectionDetails) -> Result<()> {
        let iceberg_context = IcebergContext::from_connection_details(&conn_details);
        let namespaces = iceberg_context.populate_namespaces().await?;

        self.open_connections = vec![iceberg_context];

        let namespaces = namespaces.read().unwrap().deref().clone();

        self.tx
            .send(TanicMessage::ShowNamespaces(namespaces))
            .await?;

        Ok(())
    }
}
