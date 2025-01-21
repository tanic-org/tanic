//! Iceberg Context

use iceberg::{Catalog, NamespaceIdent};
use iceberg_catalog_rest::{RestCatalog, RestCatalogConfig};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::watch::Receiver;
use tokio_stream::{wrappers::WatchStream, StreamExt};

use tanic_core::config::ConnectionDetails;
use tanic_core::message::{NamespaceDeets, TableDeets};
use tanic_core::{Result, TanicError};

use crate::state::{TanicAction, TanicAppState, ViewingNamespacesListState};

#[derive(Debug)]
enum Connection {
    Disconnected,
    Connected(IcebergContext),
}

#[derive(Debug)]
struct IcebergContext {
    connection_details: ConnectionDetails,

    /// Iceberg Catalog
    catalog: Option<Arc<dyn Catalog>>,

    namespaces: Vec<NamespaceDeets>,
    tables: Vec<TableDeets>,
}

/// Iceberg Context
#[derive(Debug)]
pub struct IcebergContextManager {
    action_tx: UnboundedSender<TanicAction>,
}

impl IcebergContextManager {
    pub fn new(action_tx: UnboundedSender<TanicAction>) -> Self {
        Self { action_tx }
    }

    pub async fn event_loop(self, state_rx: Receiver<TanicAppState>) -> Result<()> {
        let mut connection = Connection::Disconnected;

        let mut state_stream = WatchStream::new(state_rx);

        while let Some(state) = state_stream.next().await {
            match state {
                TanicAppState::ConnectingTo(ref new_conn_details) => {
                    match &mut connection {
                        // initial connection
                        Connection::Disconnected => {
                            let mut context = IcebergContext::connect_to(new_conn_details);

                            context.populate_namespaces().await?;

                            self.action_tx
                                .send(TanicAction::RetrievedNamespaceList(
                                    context.namespaces.clone(),
                                ))
                                .map_err(|err| TanicError::UnexpectedError(err.to_string()))?;

                            connection = Connection::Connected(context);
                        }

                        // already existing connection? No Op
                        Connection::Connected(IcebergContext {
                            connection_details, ..
                        }) if connection_details.uri == new_conn_details.uri => {}

                        // switch connection
                        Connection::Connected(_) => {
                            let mut context = IcebergContext::connect_to(new_conn_details);

                            context.populate_namespaces().await?;

                            self.action_tx
                                .send(TanicAction::RetrievedNamespaceList(
                                    context.namespaces.clone(),
                                ))
                                .map_err(|err| TanicError::UnexpectedError(err.to_string()))?;

                            connection = Connection::Connected(context);
                        }
                    }
                }
                TanicAppState::ViewingNamespacesList(_) => {}
                TanicAppState::RetrievingTableList(ViewingNamespacesListState {
                    namespaces,
                    selected_idx,
                }) => {
                    let namespace = &namespaces[selected_idx];
                    if let Connection::Connected(ref mut iceberg_ctx) = &mut connection {
                        iceberg_ctx.populate_table_list(&namespace.parts).await?;

                        self.action_tx
                            .send(TanicAction::RetrievedTableList(
                                namespace.clone(),
                                iceberg_ctx.tables.clone(),
                            ))
                            .map_err(|err| TanicError::UnexpectedError(err.to_string()))?;
                    }
                }
                TanicAppState::Exiting => {
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl IcebergContext {
    /// Create a new Iceberg Context from a Uri
    pub fn connect_to(connection_details: &ConnectionDetails) -> Self {
        let connection_details = connection_details.clone();

        let mut uri_str = connection_details.uri.to_string();
        uri_str.pop();

        let config = RestCatalogConfig::builder().uri(uri_str).build();
        let rest_catalog = RestCatalog::new(config);

        Self {
            connection_details,
            namespaces: vec![],
            tables: vec![],
            catalog: Some(Arc::new(rest_catalog)),
        }
    }

    pub async fn populate_namespaces(&mut self) -> Result<()> {
        let Some(ref catalog) = self.catalog else {
            panic!();
        };

        let root_namespaces = catalog.list_namespaces(None).await?;

        let namespaces = root_namespaces
            .into_iter()
            .map(|ns| {
                let parts = ns.inner();
                let name = parts.clone().join(".");
                NamespaceDeets {
                    parts,
                    name,
                    table_count: 1,
                }
            })
            .collect::<Vec<_>>();

        self.namespaces = namespaces;

        Ok(())
    }

    pub async fn populate_table_list(&mut self, namespace_parts: &Vec<String>) -> Result<()> {
        let Some(ref catalog) = self.catalog else {
            panic!();
        };

        let tables = catalog
            .list_tables(&NamespaceIdent::from_strs(namespace_parts)?)
            .await?;

        let table_names = tables
            .into_iter()
            .map(|ti| TableDeets {
                namespace: namespace_parts.clone(),
                name: ti.name().to_string(),
                row_count: 1,
            })
            .collect::<Vec<_>>();

        self.tables = table_names;

        Ok(())
    }
}
