use std::{path::PathBuf, process::Command};

use crossterm::event::KeyModifiers;
use tui::widgets::ListState;

pub struct App {
    pub should_quit: bool,
    pub favorites: StatefulList<ProjectEntry>,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            should_quit: false,
            favorites: StatefulList::with_items(vec![
                ProjectEntry::new("kaiju", PathBuf::from("/home/mike/code/cli/kaiju")),
                ProjectEntry::new(
                    "typescript",
                    PathBuf::from("/home/mike/code/microsoft/typescript"),
                ),
                ProjectEntry::new(
                    "web-lang",
                    PathBuf::from("/home/mike/code/MCluck90/web-lang"),
                ),
            ]),
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
            let _ = Command::new("code")
                .args([selected.path.to_str().unwrap()])
                .spawn()
                .unwrap();
            self.should_quit = true;
        }
    }
}

pub struct ProjectEntry {
    pub name: String,
    pub path: PathBuf,
}

impl ProjectEntry {
    pub fn new(name: &str, path: PathBuf) -> Self {
        Self {
            name: name.into(),
            path,
        }
    }
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
