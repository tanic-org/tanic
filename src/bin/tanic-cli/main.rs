use miette::{IntoDiagnostic, Result};

use crate::args::Args;
use state::TanicState;
use tanic::config::TanicConfig;

mod app;
pub mod args;
mod state;
mod tracing_subscriber;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::init();

    let (ui_tx, ui_rx) = tokio::sync::mpsc::channel(1);
    let (app_tx, app_rx) = tokio::sync::mpsc::channel(1);

    let ui_task = tokio::spawn(async move { tui::TanicTui::start(app_rx, ui_tx).await });

    let app_task = tokio::spawn(async move { app::TanicTuiApp::start(ui_rx, app_tx).await });

    tokio::select! {
        _ = ui_task => Ok(()),
        _ = app_task => Ok(())
    }
}
