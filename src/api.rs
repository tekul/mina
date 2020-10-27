use crate::db::Track;

use serde::Serialize;

pub struct Api<'a> {
    url: &'a str,
    src_url: &'a str,
    client: reqwest::blocking::Client,
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
                dbg!(track);
            }
        }
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
