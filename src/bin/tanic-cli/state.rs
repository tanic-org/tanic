//! Tanic State module
//!
//! Internal application state, not for consumption outside of the library

use crate::{args::Args, Result};
use http::Uri;
use iceberg::NamespaceIdent;
use tanic::config::{CatalogConnectionDetails, TanicConfig};
use tanic::iceberg_context::IcebergContext;

/// A currently open connection
///
/// either ephemeral or indexed into the connection library in the app config
#[derive(Debug)]
enum CurrentConnection {
    UnsavedConnection(CatalogConnectionDetails),
    SavedConnection(usize),
}

/// Global application state
#[derive(Debug)]
pub struct TanicState {
    config: TanicConfig,
    current_connections: Vec<CurrentConnection>,
    current_iceberg_context: Option<IcebergContext>,

    pub(crate) current_namespaces: Option<Vec<NamespaceIdent>>,
}

impl Default for TanicState {
    fn default() -> Self {
        TanicState {
            current_connections: vec![],
            config: TanicConfig::default(),
            current_iceberg_context: None,
            current_namespaces: None,
        }
    }
}

impl TanicState {
    pub(crate) fn from_args_and_config(args: &Args, config: TanicConfig) -> Self {
        let current_connections = if let Some(ref catalogue_uri) = args.catalogue_uri {
            let conn = CurrentConnection::UnsavedConnection(CatalogConnectionDetails {
                name: "Unnamed Connection".to_string(),
                uri: catalogue_uri.clone(),
            });

            vec![conn]
        } else if config.library.is_empty() {
            vec![]
        } else {
            let conn = CurrentConnection::SavedConnection(0);
            vec![conn]
        };

        Self {
            config,
            current_connections,
            current_iceberg_context: None,
            current_namespaces: None,
        }
    }

    pub(crate) fn current_connection_uri(&self) -> Option<Uri> {
        self.current_connections.first().map(|x| match x {
            CurrentConnection::UnsavedConnection(conn) => conn.uri.clone(),
            CurrentConnection::SavedConnection(idx) => {
                let conn = self.config.library.get(*idx).unwrap();

                conn.uri.clone()
            }
        })
    }

    pub(crate) fn init_iceberg_context(&mut self) {
        let iceberg_context = self
            .current_connection_uri()
            .map(|uri| IcebergContext::new(uri));
        tracing::info!(?iceberg_context, "iceberg context initialized");

        self.current_iceberg_context = iceberg_context;
    }

    pub(crate) fn has_iceberg_context(&self) -> bool {
        self.current_iceberg_context.is_some()
    }

    pub(crate) async fn refresh_iceberg_namespaces(&mut self) -> Result<()> {
        let Some(ref ctx) = self.current_iceberg_context else {
            return Ok(());
        };

        let root_namespaces = ctx.catalog.list_namespaces(None).await.unwrap();

        self.current_namespaces = Some(root_namespaces);

        Ok(())
    }
}
