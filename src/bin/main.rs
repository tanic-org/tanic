use miette::{IntoDiagnostic, Result};

use crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();

    Ok(())
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}
