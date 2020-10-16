use crate::app::App;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        //            .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.size());

    let artists: Vec<ListItem> = app
        .artists
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(i.name))]))
        .collect();

    let artists = List::new(artists)
        .block(Block::default().borders(Borders::ALL).title("Artists"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    //     .highlight_symbol("> ");

    let tracks: Vec<ListItem> = app
        .tracks
        .items
        .iter()
        .map(|t| ListItem::new(vec![Spans::from(Span::raw(t.title.as_str()))]))
        .collect();

    let tracks = List::new(tracks)
        .block(Block::default().borders(Borders::ALL).title("Tracks"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(artists, chunks[0], &mut app.artists.state);
    f.render_stateful_widget(tracks, chunks[1], &mut app.tracks.state);
}
