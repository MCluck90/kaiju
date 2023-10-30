use std::time::Duration;
use std::{error::Error, io};

use app::{App, ArrowKey};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use home::home_dir;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod app;
mod ui;

fn run() -> Result<(), Box<dyn Error>> {
    let config_path = home_dir()
        .map(|path| {
            let mut path = path.clone();
            path.push(".config/kaiju.json");
            path
        })
        .unwrap();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut app = App::new(config_path);
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if crossterm::event::poll(Duration::MAX)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => app.on_key(c, key.modifiers),
                    KeyCode::Up => app.on_arrow_key(ArrowKey::Up),
                    KeyCode::Down => app.on_arrow_key(ArrowKey::Down),
                    KeyCode::Left => app.on_arrow_key(ArrowKey::Left),
                    KeyCode::Right => app.on_arrow_key(ArrowKey::Right),
                    KeyCode::Enter => app.on_enter(),
                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run()?;
    Ok(())
}
