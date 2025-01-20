use clap::Parser;
use miette::{IntoDiagnostic, Result};
use std::sync::{Arc, RwLock};

use crate::args::Args;
use tanic_core::config::ConnectionDetails;
use tanic_core::{TanicConfig, TanicMessage};
use tanic_svc::{AppStateManager, TanicAction};
use tanic_tui::TanicTui;

mod args;
mod logging;
mod lifecycle;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init_tui_logger();

    let args = Args::try_parse().into_diagnostic()?;
    let config = TanicConfig::load().into_diagnostic()?;
    tracing::info!(?config, "loaded config");
    let config = Arc::new(RwLock::new(config));

    let (app_state, action_tx, state_rx) = AppStateManager::new();
    let tanic_tui = TanicTui::new(state_rx.clone(), action_tx.clone());

    let ui_task = tokio::spawn(async move { app_state.event_loop().await });
    let svc_task = tokio::spawn(async move { tanic_tui.event_loop().await });

    if let Some(ref uri) = args.catalogue_uri {
        let connection = ConnectionDetails::new_anon(uri.clone());

        let message = TanicAction::ConnectTo(connection);
        action_tx.send(message).await.into_diagnostic()?;
    }

    tokio::select! {
        _ = ui_task => Ok(()),
        _ = svc_task => Ok(())
    }
}
    