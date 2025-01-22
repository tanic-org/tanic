use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::Block;

use crate::component::Component;
use crate::ui_components::table_list_item::TableListItem;
use crate::ui_components::treemap_layout::TreeMapLayout;
use tanic_svc::{TanicAction, TanicAppState};

pub(crate) struct TableListView<'a> {
    state: &'a TanicAppState,
}

impl<'a> TableListView<'a> {
    pub(crate) fn new(state: &'a TanicAppState) -> Self {
        Self { state }
    }
}

impl Component for &TableListView<'_> {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<TanicAction> {
        match key_event.code {
            KeyCode::Left => Some(TanicAction::FocusPrevTable),
            KeyCode::Right => Some(TanicAction::FocusNextTable),
            KeyCode::Enter => Some(TanicAction::SelectTable),
            KeyCode::Esc => Some(TanicAction::LeaveNamespace),
            _ => None,
        }
    }

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let TanicAppState::ViewingTablesList(view_state) = self.state else {
            panic!();
        };

        let block = Block::bordered()
            .title(format!(
                " Tanic //// {} Namespace ",
                view_state.namespace.name
            ))
            .border_set(border::PLAIN);
        let block_inner_area = block.inner(area);

        let items = view_state
            .tables
            .iter()
            .enumerate()
            .map(|(idx, ns)| {
                TableListItem::new(ns, view_state.selected_idx.unwrap_or(usize::MAX) == idx)
            })
            .collect::<Vec<_>>();

        let children: Vec<(&TableListItem, usize)> = items
            .iter()
            .map(|item| (item, item.table.row_count))
            .collect::<Vec<_>>();

        let layout = TreeMapLayout::new(children);

        block.render(area, buf);
        (&layout).render(block_inner_area, buf);
    }
}
