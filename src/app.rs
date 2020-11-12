use crate::api;
use crate::db::Track;
use crate::widgets::StatefulList;

use tui::widgets::TableState;

#[derive(Debug, PartialEq)]
enum Pane {
    ARTISTS,
    TRACKS,
}

pub struct App<'a> {
    pub should_quit: bool,
    pub artists: StatefulList<Artist<'a>>,
    pub albums: StatefulList<&'a str>,
    pub tracks: Vec<&'a Track>,
    all_tracks: &'a Vec<Track>,
    current_pane: Pane,
    pub track_list_state: TableState,
    naim_api: api::Api<'a>,
    volume: Option<u8>,
}

pub struct Artist<'a> {
    pub name: &'a str,
}

impl<'a> App<'a> {
    pub fn new(naim_api: api::Api<'a>, tracks: &'a Vec<Track>) -> App<'a> {
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
        let current_tracks: Vec<&Track> = tracks
            .iter()
            .filter(|t| t.artist == current_artist.name)
            .collect();

        App {
            all_tracks: tracks,
            should_quit: false,
            artists: StatefulList::with_items(artists),
            albums: StatefulList::with_items(albums),
            tracks: current_tracks,
            current_pane: Pane::ARTISTS,
            track_list_state: TableState::default(),
            naim_api,
            volume: None,
        }
    }

    pub fn on_up(&mut self) {
        if self.current_pane == Pane::ARTISTS {
            self.artists.previous(1);
            self.set_tracks();
        } else {
            self.track_list_state
                .select(match self.track_list_state.selected() {
                    Some(pos) => {
                        if pos == 0 {
                            Some(self.tracks.len() - 1)
                        } else {
                            Some(pos - 1)
                        }
                    }
                    None => Some(0),
                })
        }
    }

    pub fn on_down(&mut self) {
        if self.current_pane == Pane::ARTISTS {
            self.artists.next(1);
            self.set_tracks();
        } else {
            self.select_next_track();
        }
    }

    fn select_next_track(&mut self) {
        self.track_list_state
            .select(match self.track_list_state.selected() {
                Some(pos) => {
                    if pos == self.tracks.len() - 1 {
                        Some(0)
                    } else {
                        Some(pos + 1)
                    }
                }
                None => Some(0),
            })
    }

    pub fn on_page_up(&mut self) {
        self.artists.previous(10);
        self.set_tracks();
    }

    pub fn on_page_down(&mut self) {
        self.artists.next(10);
        self.set_tracks();
    }

    fn current_track(&self) -> Option<&Track> {
        self.track_list_state
            .selected()
            .map(move |i| *self.tracks.get(i).unwrap())
        //            .map(|t| PlaylistTrack::from_track(self.src_url, t))
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            '\t' => {
                self.current_pane = match self.current_pane {
                    Pane::ARTISTS => {
                        match self.track_list_state.selected() {
                            None => self.track_list_state.select(Some(0)),
                            _ => (),
                        };
                        Pane::TRACKS
                    }
                    Pane::TRACKS => {
                        self.track_list_state.select(None);
                        Pane::ARTISTS
                    }
                }
            }
            '\n' => match self.current_track() {
                Some(track) => {
                    self.naim_api.queue_track(track);
                    self.select_next_track();
                }
                None => (),
            },
            'p' => self.naim_api.play(),
            ' ' => self.naim_api.toggle_play_pause(),
            '+' => self.volume = self.naim_api.incr_volume(self.volume),
            '-' => self.volume = self.naim_api.decr_volume(self.volume),
            'P' => self.naim_api.power_on(),
            'S' => self.naim_api.suspend(),
            'C' => self.naim_api.clear_playlist(),
            _ => {}
        }
    }

    fn set_tracks(&mut self) {
        if self.current_pane == Pane::TRACKS {
            return;
        }
        let index = self.artists.state.selected().unwrap();
        let current_artist = self.artists.items.get(index).unwrap();
        let new_tracks: Vec<&Track> = self
            .all_tracks
            .iter()
            .filter(|t| t.artist == current_artist.name)
            .collect();

        self.tracks = new_tracks;
        self.track_list_state.select(None);
    }
    pub fn on_tick(&mut self) {}
}
