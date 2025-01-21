use clap::Parser;
use miette::{IntoDiagnostic, Result};

use crate::args::Args;
use tanic_core::config::ConnectionDetails;
use tanic_core::TanicConfig;
use tanic_svc::iceberg_context::IcebergContextManager;
use tanic_svc::{AppStateManager, TanicAction};
use tanic_tui::TanicTui;

mod args;
mod lifecycle;
mod logging;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init_tui_logger();

    let args = Args::try_parse().into_diagnostic()?;
    let config = TanicConfig::load().into_diagnostic()?;
    tracing::info!(?config, "loaded config");
    // let config = Arc::new(RwLock::new(config));

    let (app_state, action_tx, state_rx) = AppStateManager::new(config);
    let tanic_tui = TanicTui::new(action_tx.clone());
    let iceberg_ctx_mgr = IcebergContextManager::new(action_tx.clone());

    let svc_task = tokio::spawn(async move { app_state.event_loop().await });
    let ui_state_rx = state_rx.clone();
    let ui_task = tokio::spawn(async move { tanic_tui.event_loop(ui_state_rx).await });
    let iceberg_task_state_rx = state_rx.clone();
    let iceberg_task =
        tokio::spawn(async move { iceberg_ctx_mgr.event_loop(iceberg_task_state_rx).await });

    if let Some(ref uri) = args.catalogue_uri {
        let connection = ConnectionDetails::new_anon(uri.clone());

        let message = TanicAction::ConnectTo(connection);
        action_tx.send(message).into_diagnostic()?;
    }

    tokio::select! {
        _ = ui_task => Ok(()),
        _ = svc_task => Ok(()),
        _ = iceberg_task => Ok(()),
    }
}
