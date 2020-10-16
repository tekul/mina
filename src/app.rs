use crate::db::Track;
use crate::widgets::StatefulList;

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub artists: StatefulList<Artist<'a>>,
    pub albums: StatefulList<&'a str>,
    pub tracks: StatefulList<&'a Track>,
    all_tracks: &'a Vec<Track>,
}

pub struct Artist<'a> {
    pub name: &'a str,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, tracks: &'a Vec<Track>) -> App<'a> {
        let mut artists = tracks.iter().map(|t| t.artist.as_str()).collect::<Vec<_>>();
        let mut albums = tracks.iter().map(|t| t.album.as_str()).collect::<Vec<_>>();
        artists.sort_unstable();
        artists.dedup();
        let artists = artists
            .iter()
            .map(|name| Artist { name })
            .collect::<Vec<_>>();
        albums.sort_unstable();
        albums.dedup();

        let current_artist = artists.get(0).unwrap();
        let current_tracks = tracks
            .iter()
            .filter(|t| t.artist == current_artist.name)
            .collect::<Vec<_>>();

        App {
            title,
            all_tracks: tracks,
            should_quit: false,
            artists: StatefulList::with_items(artists),
            albums: StatefulList::with_items(albums),
            tracks: StatefulList::with_items(current_tracks),
        }
    }

    pub fn on_up(&mut self) {
        self.artists.previous(1);
        self.set_tracks();
    }

    pub fn on_down(&mut self) {
        self.artists.next(1);
        self.set_tracks();
    }

    pub fn on_page_up(&mut self) {
        self.artists.previous(10);
        self.set_tracks();
    }

    pub fn on_page_down(&mut self) {
        self.artists.next(10);
        self.set_tracks();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    fn set_tracks(&mut self) {
        let index = self.artists.state.selected().unwrap();
        let current_artist = self.artists.items.get(index).unwrap();
        let new_tracks = self
            .all_tracks
            .iter()
            .filter(|t| t.artist == current_artist.name)
            .collect::<Vec<_>>();

        self.tracks = StatefulList::with_items(new_tracks);
    }
    pub fn on_tick(&mut self) {}
}
