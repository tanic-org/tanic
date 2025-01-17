use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use crate::tui::ui_state::ViewConnectionListState;

pub(crate) fn render_view_connection_list(state: &ViewConnectionListState, area: Rect, buf: &mut Buffer) {
    let title = Line::from(" Tanic ".bold());

    let block = Block::bordered()
        .title(title.centered())
        .border_set(border::THICK);

    Paragraph::new("TODO: Connection list")
        .centered()
        .block(block)
        .render(area, buf);
}
