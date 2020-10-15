use crate::db::Track;
use crate::widgets::StatefulList;

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub artists: StatefulList<&'a str>,
    pub albums: StatefulList<&'a str>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, tracks: &'a Vec<Track>) -> App<'a> {
        let mut artists = tracks.iter().map(|t| t.artist.as_str()).collect::<Vec<_>>();
        let mut albums = tracks.iter().map(|t| t.album.as_str()).collect::<Vec<_>>();
        artists.sort_unstable();
        artists.dedup();
        albums.sort_unstable();
        albums.dedup();

        App {
            title,
            should_quit: false,
            artists: StatefulList::with_items(artists),
            albums: StatefulList::with_items(albums),
        }
    }

    pub fn on_up(&mut self) {
        self.artists.previous();
    }

    pub fn on_down(&mut self) {
        self.artists.next();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {}
}
