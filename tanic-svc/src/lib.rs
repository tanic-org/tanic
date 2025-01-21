use tanic_core::TanicConfig;
use tanic_core::{Result, TanicError};
use tokio::sync::mpsc::{UnboundedReceiver as MpscReceiver, UnboundedSender as MpscSender};
use tokio::sync::watch::{Receiver as WatchReceiver, Sender as WatchSender};

pub mod iceberg_context;
pub mod state;

pub use state::{TanicAction, TanicAppState};

pub struct AppStateManager {
    action_rx: MpscReceiver<TanicAction>,

    #[allow(unused)]
    action_tx: MpscSender<TanicAction>,
    state_tx: WatchSender<TanicAppState>,

    state: TanicAppState,
}

impl AppStateManager {
    pub fn new(
        _config: TanicConfig,
    ) -> (Self, MpscSender<TanicAction>, WatchReceiver<TanicAppState>) {
        let state = TanicAppState::default();

        let (action_tx, action_rx) = tokio::sync::mpsc::unbounded_channel();
        let (state_tx, state_rx) = tokio::sync::watch::channel(state.clone());

        (
            Self {
                action_rx,
                action_tx: action_tx.clone(),
                state_tx,
                state,
            },
            action_tx,
            state_rx,
        )
    }

    pub async fn event_loop(self) -> Result<()> {
        let Self {
            mut state,
            state_tx,
            mut action_rx,
            ..
        } = self;

        while !matches!(state, TanicAppState::Exiting) {
            let Some(action) = action_rx.recv().await else {
                break;
            };
            tracing::info!(?action, "AppState received an action");

            let next_state = state.reduce(action);

            state = next_state;
            state_tx
                .send(state.clone())
                .map_err(|err| TanicError::UnexpectedError(err.to_string()))?;
        }

        Ok(())
    }
}
