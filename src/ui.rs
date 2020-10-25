use crate::app::App;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Row, Table},
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

    let selected_style = Style::default().add_modifier(Modifier::BOLD);

    let artists = List::new(artists)
        .block(Block::default().borders(Borders::ALL).title("Artists"))
        .highlight_style(selected_style);

    let rows = app
        .tracks
        .iter()
        .map(|i| Row::Data(vec![i.title.as_str(), i.album.as_str()].into_iter()));
    let tracks_table = Table::new(vec!["Title", "Album"].into_iter(), rows)
        .block(Block::default().borders(Borders::ALL).title("Tracks"))
        .highlight_style(selected_style)
        .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);

    f.render_stateful_widget(artists, chunks[0], &mut app.artists.state);
    f.render_stateful_widget(tracks_table, chunks[1], &mut app.track_list_state);
}
