use crate::app::App;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Row, Table},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        //            .margin(1)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(size);

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
        .map(|i| Row::new(vec![i.title.as_str(), i.album.as_str()]));
    let tracks_table = Table::new(rows)
        .header(Row::new(vec!["Title", "Album"]))
        .block(Block::default().borders(Borders::ALL).title("Tracks"))
        .highlight_style(selected_style)
        .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);
    f.render_stateful_widget(artists, chunks[0], &mut app.artists.state);
    f.render_stateful_widget(tracks_table, chunks[1], &mut app.track_list_state);

    if app.mode == crate::app::Mode::SearchInput {
        let search_box = Block::default().borders(Borders::ALL);
        let search_box_area = centered_rect(60, 20, size);
        f.render_widget(Clear, search_box_area);
        f.render_widget(search_box, search_box_area);
        let input = Paragraph::new(app.search_input.as_ref())
            .block(Block::default().title("Search").borders(Borders::ALL));
        let input_area = Rect::new(
            search_box_area.x + 4,
            search_box_area.y + search_box_area.height / 2 - 1,
            search_box_area.width - 8,
            3,
        );
        f.render_widget(input, input_area);

        f.set_cursor(
            input_area.x + app.search_input.len() as u16 + 1,
            input_area.y + 1,
        )
    }
}

/// helper function to create a centered rect using up
/// certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
