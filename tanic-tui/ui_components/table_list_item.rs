use crate::component::Component;
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use tanic_core::message::TableDeets;

const NERD_FONT_ICON_TABLE: &str = "\u{ebb7}"; // 

pub(crate) struct TableListItem<'a> {
    pub(crate) table: &'a TableDeets,
    pub(crate) is_selected: bool,
}

impl<'a> TableListItem<'a> {
    pub(crate) fn new(table: &'a TableDeets, is_selected: bool) -> Self {
        Self { table, is_selected }
    }
}

impl Component for &TableListItem<'_> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::new().border_set(border::THICK);
        let block_inner = block.inner(area);

        if self.is_selected {
            block = block.style(Style::new().bg(Color::Cyan));
        }

        let name = self.table.name.clone();
        let plural_suffix = if self.table.row_count == 1 { "" } else { "s" };
        let name = format!(
            "{} {} ({} row{})",
            NERD_FONT_ICON_TABLE, name, self.table.row_count, plural_suffix
        );

        let para_rect = Rect::new(
            block_inner.x,
            block_inner.y + (block_inner.height / 2),
            block_inner.width,
            1,
        );

        let mut para = Paragraph::new(name)
            .alignment(Alignment::Center)
            .white()
            .bold();

        if self.is_selected {
            para = para.black();
        }

        block.render(area, buf);
        para.render(para_rect, buf);
    }
}
