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

    let (app_state, action_tx, state_rx) = AppStateManager::new(config);

    let ui_task = tokio::spawn({
        let tanic_tui = TanicTui::new(action_tx.clone());
        let state_rx = state_rx.clone();
        let app_state = app_state.get_state();
        async move { tanic_tui.event_loop(state_rx, app_state).await }
    });

    let iceberg_task = tokio::spawn({
        let state_rx = state_rx.clone();
        let app_state = app_state.get_state();
        let iceberg_ctx_mgr = IcebergContextManager::new(action_tx.clone(), app_state);
        async move { iceberg_ctx_mgr.event_loop(state_rx).await }
    });

    let svc_task = tokio::spawn(async move { app_state.event_loop().await });

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
