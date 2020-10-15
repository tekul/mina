use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Track {
    pub id: u16,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_art_id: u16,
    pub track_number: u16,
    pub disc_number: u8,
    pub track_id: String,
    pub duration: String,
    pub mime_type: String,
}

pub fn read_tracks() -> Result<Vec<Track>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("tracks.csv")?;
    let mut tracks = Vec::with_capacity(10000);

    for result in rdr.deserialize() {
        let track: Track = result?;
        tracks.push(track);
    }
    Ok(tracks)
}
