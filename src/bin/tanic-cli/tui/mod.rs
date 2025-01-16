use std::io;

use crate::app::AppMessage;
use crate::tui::ui_state::TanicUiState;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use tanic::Result;
use tokio::sync::mpsc::{Receiver, Sender};

mod ui_state;

pub enum UiMessage {}

pub(crate) struct TanicTui {
    should_exit: bool,
    rx: Receiver<AppMessage>,
    tx: Sender<UiMessage>,
}

impl TanicTui {
    pub(crate) async fn start(rx: Receiver<AppMessage>, tx: Sender<UiMessage>) -> Result<()> {
        TanicTui::new(rx, tx).event_loop()
    }

    fn new(rx: Receiver<AppMessage>, tx: Sender<UiMessage>) -> Self {
        Self {
            should_exit: false,
            rx,
            tx,
        }
    }

    fn event_loop(&mut self) -> Result<()> {
        let mut terminal = ratatui::init();

        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        ratatui::restore();
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            // KeyCode::Left => self.decrement_counter(),
            // KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.should_exit = true;
    }
}

impl Widget for &TanicTui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Tanic ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec!["Hello ".into(), "World".into()])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
