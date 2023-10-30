use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::PathBuf, process::Command};

use crossterm::event::KeyModifiers;
use tui::widgets::ListState;

pub struct App {
    pub should_quit: bool,
    pub favorites: StatefulList<ProjectEntry>,
}

impl App {
    pub fn new(config_path: PathBuf) -> App {
        let config = AppConfig::load(config_path);
        let favorites = StatefulList::with_items(config.projects);
        let mut app = App {
            should_quit: false,
            favorites,
        };
        app.favorites.next(); // Auto-highlight the first one
        app
    }

    pub fn on_key(&mut self, char: char, modifiers: KeyModifiers) {
        if char == 'q' || (char == 'c' && modifiers.contains(KeyModifiers::CONTROL)) {
            self.should_quit = true;
            return;
        }

        match char {
            'j' => self.favorites.next(),
            'k' => self.favorites.prev(),
            _ => {}
        }
    }

    pub fn on_arrow_key(&mut self, key: ArrowKey) {
        match key {
            ArrowKey::Up => self.favorites.prev(),
            ArrowKey::Down => self.favorites.next(),
            _ => {}
        }
    }

    pub fn on_enter(&mut self) {
        if let Some(selected_index) = self.favorites.state.selected() {
            let selected = self.favorites.items.get(selected_index).unwrap();
            let mut home = home_dir().unwrap();
            let path = if selected.path.starts_with("~") {
                let path = selected.path.clone();
                home.push(path.strip_prefix("~").unwrap());
                &home
            } else {
                &selected.path
            };
            let _ = Command::new("code")
                .args([path.to_str().unwrap()])
                .spawn()
                .unwrap();
            self.should_quit = true;
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub projects: Vec<ProjectEntry>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
        }
    }
}

impl AppConfig {
    pub fn load(config_path: PathBuf) -> AppConfig {
        let file = match File::open(config_path) {
            Ok(file) => file,
            Err(e) => {
                println!("{:?}", e);
                return AppConfig::default();
            }
        };
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).unwrap_or_else(|_| AppConfig::default())
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProjectEntry {
    pub name: String,
    pub path: PathBuf,
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        if self.items.len() == 0 {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn prev(&mut self) {
        if self.items.len() == 0 {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub enum ArrowKey {
    Up,
    Down,
    Left,
    Right,
}
