use crate::args::Args;
use crate::state::TanicState;
use crate::tui::{TanicTui, UiMessage};
use crate::Result;
use clap::Parser;
use miette::IntoDiagnostic;
use tanic::config::TanicConfig;
use tokio::sync::mpsc::{Receiver, Sender};

pub enum AppMessage {}

pub(crate) struct TanicTuiApp {
    should_exit: bool,
    rx: Receiver<UiMessage>,
    tx: Sender<AppMessage>,
}

impl TanicTuiApp {
    pub(crate) async fn start(rx: Receiver<UiMessage>, tx: Sender<AppMessage>) -> Result<()> {
        TanicTuiApp::new(rx, tx).event_loop().await
    }

    fn new(rx: Receiver<UiMessage>, tx: Sender<AppMessage>) -> Self {
        Self {
            should_exit: false,
            rx,
            tx,
        }
    }

    pub async fn event_loop(&mut self) -> Result<()> {
        let args = Args::parse();
        let app_config = TanicConfig::load().into_diagnostic()?;
        tracing::info!(?app_config, "loaded config");

        let mut state = TanicState::from_args_and_config(&args, app_config);
        tracing::info!(?state, "initial state");

        state.init_iceberg_context();
        if state.has_iceberg_context() {
            state.refresh_iceberg_namespaces().await?;
        }
        tracing::info!(root_namespaces = ?state.current_namespaces);

        while !self.should_exit {
            let message = self.rx.recv().await;
        }

        Ok(())
    }
}
