pub mod fish;
pub mod player;
pub mod app;

use std::io;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = draw(terminal); 
    ratatui::restore();
    app_result
}


fn draw(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut text = "Main Menu";
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new(text).white().on_blue();
            frame.render_widget(greeting, frame.area());
        })?;
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('f') {
                text = "fishing";
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('s') {
                text = "shop";
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc {
                text = "Main Menu";
            }

        }
    }
}