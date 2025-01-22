use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::canvas::{Canvas, Rectangle};
use ratatui::widgets::Block;
use treemap::{MapItem, Mappable, Rect as TreeMapRect, TreemapLayout};

use tanic_svc::{TanicAction, TanicAppState};

// find more at https://www.nerdfonts.com/cheat-sheet
const NERD_FONT_ICON_TABLE: &str = "\u{ebb7}"; // 

pub(crate) struct TableListView<'a> {
    state: &'a TanicAppState,
}

impl<'a> TableListView<'a> {
    pub(crate) fn new(state: &'a TanicAppState) -> Self {
        Self { state }
    }

    pub(crate) fn handle_key_event(&self, key_event: KeyEvent) -> Option<TanicAction> {
        match key_event.code {
            KeyCode::Left => Some(TanicAction::FocusPrevTable),
            KeyCode::Right => Some(TanicAction::FocusNextTable),
            KeyCode::Enter => Some(TanicAction::SelectTable),
            KeyCode::Esc => Some(TanicAction::LeaveNamespace),
            _ => None,
        }
    }
}

impl Widget for &TableListView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = TreemapLayout::new();
        let bounds = TreeMapRect::from_points(
            area.x as f64,
            area.y as f64,
            area.width as f64,
            area.height as f64,
        );

        let TanicAppState::ViewingTablesList(view_state) = self.state else {
            panic!();
        };

        let mut items: Vec<Box<dyn Mappable>> = view_state
            .tables
            .iter()
            .map(|table| {
                let res: Box<dyn Mappable> =
                    Box::new(MapItem::with_size(table.row_count.max(1) as f64));
                res
            })
            .collect::<Vec<_>>();

        layout.layout_items(&mut items, bounds);

        let selected_idx = view_state.selected_idx;

        let canvas = Canvas::default()
            .block(Block::bordered().title(format!(
                " Tanic //// {} Namespace ",
                view_state.namespace.name
            )))
            .x_bounds([area.x as f64, (area.x + area.width) as f64])
            .y_bounds([area.y as f64, (area.y + area.height) as f64])
            .paint(|ctx| {
                for (idx, item) in items.iter().enumerate() {
                    let item_bounds = item.bounds();

                    let rect = Rectangle {
                        x: item_bounds.x,
                        y: item_bounds.y,
                        width: item_bounds.w,
                        height: item_bounds.h,
                        color: Color::White,
                    };

                    ctx.draw(&rect);

                    let style = if Some(idx) == selected_idx {
                        Style::new().black().bold().on_white()
                    } else {
                        Style::new().white()
                    };

                    let name = view_state.tables[idx].name.clone();
                    let name = format!("{} {}", NERD_FONT_ICON_TABLE, name);

                    let name_len = name.len();
                    let text = Line::styled(name, style);

                    ctx.print(
                        item_bounds.x + (item_bounds.w * 0.5) - (name_len as f64 * 0.5),
                        item_bounds.y + (item_bounds.h * 0.5),
                        text,
                    );
                }
            });

        canvas.render(area, buf);
    }
}
