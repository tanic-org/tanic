use crate::component::Component;
use crate::ui_components::{
    namespace_list_view::NamespaceListView, splash_screen::SplashScreen,
    table_list_view::TableListView,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Color, Style, Widget};
use ratatui::widgets::Block;
use tanic_svc::{TanicAction, TanicAppState};
use tui_logger::{LevelFilter, TuiLoggerLevelOutput, TuiLoggerWidget, TuiWidgetState};

pub(crate) struct AppContainer<'a> {
    state: &'a TanicAppState,
    namespace_list_view: NamespaceListView<'a>,
    table_list_view: TableListView<'a>,
    splash_screen: SplashScreen<'a>,
}

impl<'a> AppContainer<'a> {
    pub(crate) fn new(state: &'a TanicAppState) -> Self {
        Self {
            state,

            namespace_list_view: NamespaceListView::new(state),
            table_list_view: TableListView::new(state),
            splash_screen: SplashScreen::new(state),
        }
    }

    pub(crate) fn handle_key_event(&self, key_event: KeyEvent) -> Option<TanicAction> {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                ..
            } => {
                // User pressed Q. Dispatch an exit action
                Some(TanicAction::Exit)
            }
            key_event => match &self.state {
                TanicAppState::ViewingNamespacesList(_) => {
                    (&self.namespace_list_view).handle_key_event(key_event)
                }
                TanicAppState::ViewingTablesList(_) => {
                    (&self.table_list_view).handle_key_event(key_event)
                }
                _ => None,
            },
        }
    }
}

impl Widget for &AppContainer<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [top, bottom] = Layout::vertical([Constraint::Fill(1), Constraint::Max(6)]).areas(area);

        let filter_state = TuiWidgetState::new()
            .set_default_display_level(LevelFilter::Info)
            .set_level_for_target("tanic_svc", LevelFilter::Debug);

        TuiLoggerWidget::default()
            .block(Block::bordered().title("Log"))
            .output_separator('|')
            .output_timestamp(Some("%F %H:%M:%S%.3f".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Long))
            .output_target(false)
            .output_file(false)
            .output_line(false)
            .style(Style::default().fg(Color::White))
            .state(&filter_state)
            .render(bottom, buf);

        match &self.state {
            TanicAppState::Initializing => self.splash_screen.render(top, buf),
            TanicAppState::ViewingNamespacesList(_) => (&self.namespace_list_view).render(top, buf),
            TanicAppState::ViewingTablesList(_) => (&self.table_list_view).render(top, buf),
            TanicAppState::Exiting => {}
            _ => {}
        }
    }
}
