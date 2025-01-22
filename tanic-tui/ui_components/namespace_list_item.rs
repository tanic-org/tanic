use crate::component::Component;
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use tanic_core::message::NamespaceDeets;

const NERD_FONT_ICON_TABLE_FOLDER: &str = "\u{f12e4}"; // 󱋤

pub(crate) struct NamespaceListItem<'a> {
    pub(crate) ns: &'a NamespaceDeets,
    pub(crate) is_selected: bool,
}

impl<'a> NamespaceListItem<'a> {
    pub(crate) fn new(ns: &'a NamespaceDeets, is_selected: bool) -> Self {
        Self { ns, is_selected }
    }
}

impl Component for &NamespaceListItem<'_> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::new().border_set(border::THICK);
        let block_inner = block.inner(area);

        if self.is_selected {
            block = block.style(Style::new().bg(Color::Cyan));
        }

        let name = self.ns.name.clone();
        let plural_suffix = if self.ns.table_count == 1 { "" } else { "s" };
        let name = format!(
            "{} {} ({} table{})",
            NERD_FONT_ICON_TABLE_FOLDER, name, self.ns.table_count, plural_suffix
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
