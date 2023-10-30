use crossterm::event::KeyModifiers;
use tui::widgets::ListState;

pub struct App {
    pub should_quit: bool,
    pub favorites: StatefulList<String>,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            should_quit: false,
            favorites: StatefulList::with_items(vec![
                "kaiju".into(),
                "typescript".into(),
                "web-lang".into(),
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
