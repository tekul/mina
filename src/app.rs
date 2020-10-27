use crate::db::Track;
use crate::widgets::StatefulList;

use serde::Serialize;
use tui::widgets::TableState;

#[derive(Debug, PartialEq)]
enum Pane {
    ARTISTS,
    TRACKS,
}

pub struct App<'a> {
    pub src_url: &'a str,
    pub dest_url: &'a str,
    pub should_quit: bool,
    pub artists: StatefulList<Artist<'a>>,
    pub albums: StatefulList<&'a str>,
    pub tracks: Vec<&'a Track>,
    all_tracks: &'a Vec<Track>,
    current_pane: Pane,
    pub track_list_state: TableState,
    client: reqwest::blocking::Client,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct PlaylistTrack<'a> {
    name: &'a str,
    artistName: &'a str,
    albumName: &'a str,
    class: &'a str,
    artwork: String,
    genre: &'static str,
    track: &'a str,
    mimeType: &'a str,
    serverId: &'a str,
    uri: String,
}

fn artwork_url(dlna_url: &str, track: &Track) -> String {
    if track.album_art_id == 0 {
        String::new()
    } else {
        let mut url = dlna_url.to_string();
        url.push_str("/AlbumArt/");
        url.push_str(&track.album_art_id.to_string());
        url.push_str("-");
        url.push_str(&track.id.to_string());
        dbg!(&url);
        url
    }
}

fn track_url(dlna_url: &str, track: &Track) -> String {
    let extension = match track.mime_type.as_str() {
        "audio/x-flac" => ".flac",
        "audio/mpeg" => ".mp3",
        "audio/mp4" => ".m4a",
        _ => ".flac",
    };
    let mut url = dlna_url.to_string();
    url.push_str("/MediaItems/");
    url.push_str(&track.id.to_string());
    url.push_str(&extension);
    dbg!(&url);
    url
}

impl<'a> PlaylistTrack<'a> {
    fn from_track(dlna_url: &str, track: &'a Track) -> Self {
        PlaylistTrack {
            name: track.title.as_str(),
            artistName: track.artist.as_str(),
            albumName: track.album.as_str(),
            class: "object.track.upnp",
            artwork: artwork_url(dlna_url, track),
            genre: "",
            track: track.track_id.as_str(),
            mimeType: track.mime_type.as_str(),
            serverId: "4d696e69-444c-164e-9d41-0001c0059ea7",
            uri: track_url(dlna_url, track),
        }
    }
}

pub struct Artist<'a> {
    pub name: &'a str,
}

impl<'a> App<'a> {
    pub fn new(src_url: &'a str, dest_url: &'a str, tracks: &'a Vec<Track>) -> App<'a> {
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
            src_url,
            dest_url,
            all_tracks: tracks,
            should_quit: false,
            artists: StatefulList::with_items(artists),
            albums: StatefulList::with_items(albums),
            tracks: current_tracks,
            current_pane: Pane::ARTISTS,
            track_list_state: TableState::default(),
            client: reqwest::blocking::Client::new(),
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
    }

    pub fn on_page_up(&mut self) {
        self.artists.previous(10);
        self.set_tracks();
    }

    pub fn on_page_down(&mut self) {
        self.artists.next(10);
        self.set_tracks();
    }

    fn current_track(&self) -> Option<PlaylistTrack> {
        self.track_list_state
            .selected()
            .map(move |i| *self.tracks.get(i).unwrap())
            .map(|t| PlaylistTrack::from_track(self.src_url, t))
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
                Some(track) => self.play_track(track),
                None => (),
            },
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

    fn play_track(&self, track: PlaylistTrack) {
        let resp = self
            .client
            .post(format!("{}/inputs/playqueue?where=end?clear=false", self.dest_url).as_str())
            .json(&track)
            .send();
        match resp {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error {}", e);
                dbg!(track);
            }
        }
    }
}
