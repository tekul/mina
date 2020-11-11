use crate::db::Track;

use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;

pub struct Api<'a> {
    url: &'a str,
    src_url: &'a str,
    client: reqwest::blocking::Client,
}

enum PowerState {
    On,
    Suspend,
}

impl<'a> Api<'a> {
    pub fn new(url: &'a str, src_url: &'a str) -> Self {
        Api {
            url,
            src_url,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn play_track(&self, track: &Track) {
        let resp = self
            .client
            .post(format!("{}/inputs/playqueue?where=end?clear=false", self.url).as_str())
            .json(&PlaylistTrack::from_track(self.src_url, track))
            .send();
        match resp {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error {}", e);
            }
        }
    }

    pub fn get_volume(&self) -> Result<u8, Box<dyn Error>> {
        let res: HashMap<String, String> = self
            .client
            .get(format!("{}/levels", self.url).as_str())
            .send()?
            .json()?;
        let volume = res
            .get("volume")
            .ok_or("'volume' not found in levels object")?
            .parse::<u8>()
            .unwrap();
        Ok(volume)
    }

    pub fn incr_volume(&self, current: Option<u8>) -> Option<u8> {
        let current_volume = current.or(self.get_volume().ok())?;

        if current_volume < 100 {
            self.set_volume(current_volume + 1)
        } else {
            current
        }
    }

    pub fn decr_volume(&self, current: Option<u8>) -> Option<u8> {
        let current_volume = current.or(self.get_volume().ok())?;

        if current_volume > 0 {
            self.set_volume(current_volume - 1)
        } else {
            current
        }
    }

    fn set_volume(&self, volume: u8) -> Option<u8> {
        self.client
            .put(format!("{}/levels?volume={}", self.url, volume).as_str())
            .send()
            .ok()
            .map(|_| volume)
    }

    pub fn power_on(&self) {
        self.power(PowerState::On);
    }

    pub fn suspend(&self) {
        self.power(PowerState::Suspend);
    }

    fn power(&self, power: PowerState) {
        let system = match power {
            PowerState::On => "on",
            PowerState::Suspend => "lona",
        };
        let _response = self
            .client
            .put(format!("{}/power?system={}", self.url, system).as_str())
            .send()
            .map_err(|e| eprintln!("{}", e));
    }
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
