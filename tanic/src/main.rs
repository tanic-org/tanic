use clap::Parser;
use miette::{IntoDiagnostic, Result};
use std::sync::Arc;

use crate::args::Args;
use tanic_core::{TanicConfig, TanicMessage};
use tanic_svc::TanicSvc;
use tanic_tui::TanicTui;

mod args;
mod logging;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init();

    let args = Args::try_parse().into_diagnostic()?;
    let app_config = TanicConfig::load().into_diagnostic()?;
    tracing::info!(?app_config, "loaded config");
    let app_config = Arc::new(app_config);

    let (ui_tx, ui_rx) = tokio::sync::mpsc::channel(1);
    let (app_tx, app_rx) = tokio::sync::mpsc::channel(1);
    let svc_app_tx = app_tx.clone();

    let ui_task = tokio::spawn(async move { TanicTui::start(app_rx, ui_tx).await });

    let svc_task = tokio::spawn(async move { TanicSvc::start(ui_rx, app_tx, app_config).await });

    if let Some(ref uri) = args.catalogue_uri {
        svc_app_tx
            .send(TanicMessage::ConnectionByUriSelected(uri.to_string()))
            .await
            .into_diagnostic()?;
    }

    tokio::select! {
        _ = ui_task => Ok(()),
        _ = svc_task => Ok(())
    }
}
