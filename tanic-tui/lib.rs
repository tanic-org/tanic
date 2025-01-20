use crate::ui_state::{NamespaceTreeViewState, TableTreeViewState, TanicUiState};
use crossterm::event::{self, Event, EventStream, KeyCode, KeyEvent, KeyEventKind};
use futures::stream::StreamExt;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget, Frame};
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::Block;
use tanic_core::TanicMessage;
use tokio::sync::Mutex;

use crate::connection_list::render_view_connection_list;
use crate::connection_prompt::render_view_connection_prompt;
use crate::initializing::render_view_initializing;
use crate::namespace_tree_view::render_namespace_treeview;
use crate::table_tree_view::render_table_treeview;
use tanic_core::message::{NamespaceDeets, TableDeets};
use tanic_core::Result;
use tanic_svc::{TanicAction,TanicAppState};
use tokio::sync::mpsc::{Receiver as MpscReceiver, Sender as MpscSender};
use tokio::sync::watch::{Receiver as WatchReceiver, Sender as WatchSender};
use tui_logger::{LevelFilter, TuiLoggerLevelOutput, TuiLoggerWidget, TuiWidgetState};

mod connection_list;
mod connection_prompt;
mod initializing;
mod namespace_tree_view;
mod table_tree_view;
mod ui_state;
mod ui_root;

use ui_root::UiRoot;

pub struct TanicTui {
    action_tx: MpscSender<TanicAction>,
}

impl TanicTui {
    pub fn new(action_tx: MpscSender<TanicAction>) -> Self {
        Self {
            action_tx,
        }
    }

    async fn event_loop(
        self,
        mut state_rx: WatchReceiver<TanicAppState>,
    ) -> Result<()> {
        let mut terminal = ratatui::init();
        let mut term_event_stream = EventStream::new();
        let mut state = TanicAppState::Initializing;
        let mut ui = {
            UiRoot::new(&state)
        };

        while !matches!(&state, TanicAppState::Exiting) {
            terminal.draw(|frame| self.draw(frame, &ui))?;

            tokio::select! {
                // Catch and handle crossterm events
                maybe_event = term_event_stream.next() => match maybe_event {
                    Some(Ok(Event::Key(key)))  => {
                        if let Some(action) = ui.handle_key_event(key) {
                            self.action_tx.send(action).await?;
                        }
                    },
                    None => break,
                    _ => (),
                },

                // Handle state updates
                Some(new_state) = state_rx.next() => {
                    state = new_state;
                },
            }
        }

        ratatui::restore();
        Ok(())
    }

    fn draw(&self, frame: &mut Frame, ui: &UiRoot) {
        frame.render_widget(self, frame.area());
    }

    // fn nav_left(&self) {
    //     let mut state = self.state.write().unwrap();
    //     match state.deref() {
    //         TanicUiState::NamespaceTreeView(NamespaceTreeViewState {
    //             selected_idx,
    //             namespaces,
    //         }) => {
    //             let selected_idx = if *selected_idx == 0 {
    //                 namespaces.len() - 1
    //             } else {
    //                 selected_idx - 1
    //             };
    //
    //             *state = TanicUiState::NamespaceTreeView(NamespaceTreeViewState {
    //                 selected_idx,
    //                 namespaces: namespaces.clone(),
    //             })
    //         }
    //         _ => {}
    //     }
    // }
    //
    // fn nav_right(&self) {
    //     let mut state = self.state.write().unwrap();
    //     match state.deref() {
    //         TanicUiState::NamespaceTreeView(NamespaceTreeViewState {
    //             selected_idx,
    //             namespaces,
    //         }) => {
    //             let selected_idx = if *selected_idx == namespaces.len() - 1 {
    //                 0
    //             } else {
    //                 selected_idx + 1
    //             };
    //
    //             *state = TanicUiState::NamespaceTreeView(NamespaceTreeViewState {
    //                 selected_idx,
    //                 namespaces: namespaces.clone(),
    //             })
    //         }
    //         _ => {}
    //     }
    // }
    //
    // fn show_namespaces(&self, namespaces: Vec<NamespaceDeets>) {
    //     let mut state = self.state.write().unwrap();
    //
    //     let selected_idx = if let TanicUiState::NamespaceTreeView(NamespaceTreeViewState {
    //         selected_idx,
    //         ..
    //     }) = state.deref()
    //     {
    //         *selected_idx
    //     } else {
    //         0
    //     };
    //
    //     *state = TanicUiState::NamespaceTreeView(NamespaceTreeViewState {
    //         namespaces,
    //         selected_idx,
    //     });
    // }
    //
    // fn show_tables_for_namespace(&self, namespace: String, tables: Vec<TableDeets>) {
    //     let mut state = self.state.write().unwrap();
    //
    //     let selected_idx =
    //         if let TanicUiState::TableTreeView(TableTreeViewState { selected_idx, .. }) =
    //             state.deref()
    //         {
    //             *selected_idx
    //         } else {
    //             0
    //         };
    //
    //     *state = TanicUiState::TableTreeView(TableTreeViewState {
    //         namespace,
    //         selected_idx,
    //         tables,
    //     });
    // }
}

struct NamespaceListView<'a> {
    state: &'a TanicAppState,
}

impl NamespaceListView {
    fn new(state: &TanicAppState) -> Self {
        Self {
            state,
        }
    }

    fn handle_key_event(&self, key_event: KeyEvent) -> Option<TanicAction> {
        match key_event.code {
            KeyCode::Left => {
                Some(TanicAction::FocusPrevNamespace)
            },
            KeyCode::Right => {
                Some(TanicAction::FocusNextNamespace)
            },
            KeyCode::Enter => {
                Some(TanicAction::SelectNamespace)
            },
            _ => None
        }
    }
}

