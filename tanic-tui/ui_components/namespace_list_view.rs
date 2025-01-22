use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::Block;

use crate::component::Component;
use crate::ui_components::namespace_list_item::NamespaceListItem;
use crate::ui_components::treemap_layout::TreeMapLayout;
use tanic_svc::{TanicAction, TanicAppState};

pub(crate) struct NamespaceListView<'a> {
    state: &'a TanicAppState,
}

impl<'a> NamespaceListView<'a> {
    pub(crate) fn new(state: &'a TanicAppState) -> Self {
        Self { state }
    }
}

impl Component for &NamespaceListView<'_> {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<TanicAction> {
        match key_event.code {
            KeyCode::Left => Some(TanicAction::FocusPrevNamespace),
            KeyCode::Right => Some(TanicAction::FocusNextNamespace),
            KeyCode::Enter => Some(TanicAction::SelectNamespace),
            _ => None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(" Tanic //// Root Namespaces")
            .border_set(border::PLAIN);
        let block_inner_area = block.inner(area);

        let TanicAppState::ViewingNamespacesList(view_state) = self.state else {
            panic!();
        };

        let items = view_state
            .namespaces
            .iter()
            .enumerate()
            .map(|(idx, ns)| {
                NamespaceListItem::new(ns, view_state.selected_idx.unwrap_or(usize::MAX) == idx)
            })
            .collect::<Vec<_>>();

        let children: Vec<(&NamespaceListItem, usize)> = items
            .iter()
            .map(|item| (item, item.ns.table_count))
            .collect::<Vec<_>>();

        let layout = TreeMapLayout::new(children);

        block.render(area, buf);
        (&layout).render(block_inner_area, buf);
    }
}
