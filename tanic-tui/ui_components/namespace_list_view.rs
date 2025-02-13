use std::sync::{Arc, RwLock};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::symbols::border;
use ratatui::widgets::Block;

use crate::component::Component;
use crate::ui_components::namespace_list_item::NamespaceListItem;
use crate::ui_components::treemap_layout::TreeMapLayout;
use tanic_svc::{TanicAction, TanicAppState};
use tanic_svc::state::{TanicIcebergState, TanicUiState};

pub(crate) struct NamespaceListView {
    state: Arc<RwLock<TanicAppState>>,
}

impl NamespaceListView {
    pub(crate) fn new(state: Arc<RwLock<TanicAppState>>) -> Self {
        Self { state }
    }
}

impl Component for &NamespaceListView {
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

        let state = self.state.read().unwrap();
        let items = self.get_items(&state);

        let children: Vec<(&NamespaceListItem, usize)> = items
            .iter()
            .map(|item| {
                let tables = &item.ns.tables;
                let table_count = tables.as_ref().map(|t|t.len()).unwrap_or(0);

                (item, table_count)
            })
            .collect::<Vec<_>>();

        let layout = TreeMapLayout::new(children);

        block.render(area, buf);
        (&layout).render(block_inner_area, buf);
    }
}

impl NamespaceListView {
    fn get_items<'a>(&self, state: &'a TanicAppState) -> Vec<NamespaceListItem<'a>> {
        // let state = self.state.read().unwrap();

        let TanicIcebergState::Connected(ref iceberg_state) = state.iceberg else {
            return vec![];
        };

        let TanicUiState::ViewingNamespacesList(ref view_state) = state.ui else {
            return vec![];
        };

        let items = iceberg_state
            .namespaces
            .iter()
            .enumerate()
            .map(|(idx, (_, ns))| {
                NamespaceListItem::new(ns, view_state.selected_idx.unwrap_or(usize::MAX) == idx)
            })
            .collect::<Vec<_>>();

        items
    }
}
