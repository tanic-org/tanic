use crate::ui_state::ViewNamespaceTreeViewState;
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};

pub(crate) struct NamespaceTreeviewState {
    items: Vec<NamespaceTreeviewItem>,
    selected: Option<usize>,
}

pub(crate) struct NamespaceTreeviewItem {
    name: String,
    size: usize,
}

pub(crate) fn render_namespace_treeview(
    view_state: &ViewNamespaceTreeViewState,
    area: Rect,
    buf: &mut Buffer,
) {
    let title = Line::from(" Tanic ".bold());

    let block = Block::bordered()
        .title(title.centered())
        .border_set(border::THICK);

    Paragraph::new("TODO: Connection list")
        .centered()
        .block(block)
        .render(area, buf);
}
