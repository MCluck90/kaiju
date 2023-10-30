use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let favorites: Vec<ListItem> = app
        .favorites
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(i.clone())];
            ListItem::new(lines)
        })
        .collect();

    let favorites = List::new(favorites)
        .block(Block::default().borders(Borders::ALL).title("Favorites"))
        .highlight_style(
            Style::default()
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(favorites, f.size(), &mut app.favorites.state);
}
