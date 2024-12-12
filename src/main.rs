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

use app::{App, CurrentScene};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    
    let mut app = App {
        exit: false,
        scene: CurrentScene::MainMenu,
        player: player::Player::new(),
    };

    while !app.exit {
        terminal.draw(|frame| app.draw(frame))?;
        app.handle_events()?;
    }
    app.run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
