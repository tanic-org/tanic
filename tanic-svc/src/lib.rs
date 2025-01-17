use state::TanicState;
use std::sync::Arc;
use tanic_core::message::TanicMessage;
use tanic_core::Result;
use tanic_core::TanicConfig;
use tokio::sync::mpsc::{Receiver, Sender};

pub mod iceberg_context;
mod state;

pub struct TanicSvc {
    should_exit: bool,
    rx: Receiver<TanicMessage>,
    tx: Sender<TanicMessage>,

    state: TanicState,
}

impl TanicSvc {
    pub async fn start(
        rx: Receiver<TanicMessage>,
        tx: Sender<TanicMessage>,
        config: Arc<TanicConfig>,
    ) -> Result<()> {
        let state = TanicState::from_config(&config);
        tracing::info!(?state, "initial state");

        let mut svc = TanicSvc::new(rx, tx, state);

        svc.state.init_iceberg_context();
        if svc.state.has_iceberg_context() {
            svc.state.refresh_iceberg_namespaces().await?;
        }
        tracing::info!(root_namespaces = ?svc.state.current_namespaces);

        svc.event_loop().await
    }

    fn new(rx: Receiver<TanicMessage>, tx: Sender<TanicMessage>, state: TanicState) -> Self {
        Self {
            state,
            should_exit: false,
            rx,
            tx,
        }
    }

    pub async fn event_loop(&mut self) -> Result<()> {
        while !self.should_exit {
            let Some(message) = self.rx.recv().await else {
                break;
            };

            match message {
                TanicMessage::Exit => {
                    self.should_exit = true;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
