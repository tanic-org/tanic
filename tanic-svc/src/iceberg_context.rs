//! Iceberg Context

use iceberg::{Catalog, NamespaceIdent};
use iceberg_catalog_rest::{RestCatalog, RestCatalogConfig};
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::watch::Receiver;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio_stream::{wrappers::WatchStream, StreamExt};

use tanic_core::config::ConnectionDetails;
use tanic_core::message::{NamespaceDeets, TableDeets};
use tanic_core::{Result, TanicError};

use crate::state::{TanicAction, TanicAppState, ViewingNamespacesListState};

type ActionTx = UnboundedSender<TanicAction>;
type IceCtxRef = Arc<RwLock<IcebergContext>>;

#[derive(Debug, Default)]
struct IcebergContext {
    connection_details: Option<ConnectionDetails>,

    /// Iceberg Catalog
    catalog: Option<Arc<dyn Catalog>>,

    namespaces: Vec<NamespaceDeets>,
    tables: Vec<TableDeets>,

    pub cancellable_action: Option<JoinHandle<()>>,
}

/// Iceberg Context
#[derive(Debug)]
pub struct IcebergContextManager {
    action_tx: ActionTx,
    iceberg_context: IceCtxRef,
}

impl IcebergContextManager {
    pub fn new(action_tx: ActionTx) -> Self {
        Self {
            action_tx,
            iceberg_context: Arc::new(RwLock::new(IcebergContext::default())),
        }
    }

    pub async fn event_loop(&self, state_rx: Receiver<TanicAppState>) -> Result<()> {
        let mut state_stream = WatchStream::new(state_rx);

        while let Some(state) = state_stream.next().await {
            match state {
                TanicAppState::ConnectingTo(ref new_conn_details) => {
                    self.connect_to(new_conn_details).await?;
                }

                TanicAppState::RetrievingTableList(ViewingNamespacesListState {
                    namespaces,
                    selected_idx,
                }) => {
                    let Some(selected_idx) = selected_idx else {
                        continue;
                    };
                    let namespace = namespaces[selected_idx].parts.clone();

                    // spawn a task to start populating the namespaces
                    let action_tx = self.action_tx.clone();
                    let ctx = self.iceberg_context.clone();

                    // TODO: handle handle, lol
                    let _jh = tokio::spawn(async move {
                        Self::populate_tables(ctx, action_tx, namespace).await
                    });
                }

                TanicAppState::Exiting => {
                    break;
                }

                _ => {}
            }
        }

        Ok(())
    }

    async fn connect_to(&self, new_conn_details: &ConnectionDetails) -> Result<()> {
        {
            let ctx = self.iceberg_context.read().await;
            if let Some(ref existing_conn_details) = ctx.connection_details {
                if new_conn_details == existing_conn_details {
                    // do nothing, already connected to this catalog
                    return Ok(());
                }
            }
        }

        // cancel any in-progress action and connect to the new connection
        {
            let mut ctx = self.iceberg_context.write().await;
            // TODO: cancel in-prog action
            // if let Some(cancellable) = *ctx.deref_mut().cancellable_action {
            //     cancellable.abort();
            // }
            ctx.connect_to(new_conn_details);
        }

        // spawn a task to start populating the namespaces
        let action_tx = self.action_tx.clone();
        let ctx = self.iceberg_context.clone();
        let jh = tokio::spawn(async move {
            Self::populate_namespaces(ctx.clone(), action_tx.clone()).await;
        });

        Ok(())
    }

    async fn populate_namespaces(ctx: IceCtxRef, action_tx: ActionTx) -> Result<()> {
        let root_namespaces = {
            let r_ctx = ctx.read().await;

            let Some(ref catalog) = r_ctx.catalog else {
                return Err(TanicError::unexpected(
                    "Attempted to populate namespaces when catalog not initialised",
                ));
            };

            catalog.list_namespaces(None).await?
        };

        let namespaces = root_namespaces
            .into_iter()
            .map(|ns| NamespaceDeets::from_parts(ns.inner()))
            .collect::<Vec<_>>();

        {
            let namespaces = namespaces.clone();
            ctx.write().await.namespaces = namespaces;
        }

        action_tx
            .send(TanicAction::RetrievedNamespaceList(namespaces))
            .map_err(|err| TanicError::UnexpectedError(err.to_string()))?;

        Ok(())
    }

    async fn populate_tables(
        ctx: IceCtxRef,
        action_tx: ActionTx,
        namespace: Vec<String>,
    ) -> Result<()> {
        let namespace_ident = NamespaceIdent::from_strs(namespace.clone())?;
        let tables = {
            let r_ctx = ctx.read().await;

            let Some(ref catalog) = r_ctx.catalog else {
                return Err(TanicError::unexpected(
                    "Attempted to populate namespaces when catalog not initialised",
                ));
            };

            catalog.list_tables(&namespace_ident).await?
        };

        let tables = tables
            .into_iter()
            .map(|ti| TableDeets {
                namespace: namespace.clone(),
                name: ti.name().to_string(),
                row_count: 1,
            })
            .collect::<Vec<_>>();

        {
            let tables = tables.clone();
            ctx.write().await.tables = tables;
        }

        action_tx
            .send(TanicAction::RetrievedTableList(
                NamespaceDeets::from_parts(namespace),
                tables,
            ))
            .map_err(TanicError::unexpected)?;

        Ok(())
    }
}

impl IcebergContext {
    /// Create a new Iceberg Context from a Uri
    pub fn connect_to(&mut self, connection_details: &ConnectionDetails) -> () {
        self.connection_details = Some(connection_details.clone());

        let mut uri_str = connection_details.uri.to_string();
        uri_str.pop();

        let config = RestCatalogConfig::builder().uri(uri_str).build();
        self.catalog = Some(Arc::new(RestCatalog::new(config)));

        self.namespaces = vec![];
        self.tables = vec![];
    }
}
