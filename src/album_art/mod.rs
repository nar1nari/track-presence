use std::{collections::HashMap, fmt::Display};

use crate::track::{ReleaseHash, Track};

#[cfg(feature = "musicbrainz")]
pub mod music_brainz;
#[cfg(feature = "musicbrainz")]
use music_brainz::MusicBrainz;

#[derive(Debug, PartialEq)]
pub enum AlbumArtError {
    Network,
    InvalidResponse,
}

impl Display for AlbumArtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network => write!(f, "internet connection broken"),
            Self::InvalidResponse => write!(f, "got invalid response from MusicBrainz"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AlbumArt(String);

impl Default for AlbumArt {
    fn default() -> Self {
        Self(String::from("icon"))
    }
}

impl AlbumArt {
    pub fn from(url: impl Into<String>) -> Self {
        Self(url.into())
    }

    pub fn url(&self) -> &str {
        &self.0
    }
}

pub trait AlbumArtSource {
    fn get_album_art(&self, track: &Track) -> Result<Option<AlbumArt>, AlbumArtError>;
}

#[derive(Default)]
pub struct AlbumArtProvider {
    cache: HashMap<ReleaseHash, AlbumArt>,

    #[cfg(feature = "musicbrainz")]
    source: MusicBrainz,
}

impl AlbumArtProvider {
    pub fn get_album_art(&mut self, track: &Track) -> AlbumArt {
        let release_hash = track.release_hash();

        if let Some(art) = self.cache.get(&release_hash) {
            return art.clone();
        }

        self.fetch_album_art(track)
    }
}

#[cfg(feature = "musicbrainz")]
impl AlbumArtProvider {
    fn fetch_album_art(&mut self, track: &Track) -> AlbumArt {
        match self.source.get_album_art(track) {
            Ok(Some(art)) => {
                self.cache.insert(track.release_hash(), art.clone());
                art
            }
            Ok(None) => {
                self.cache.insert(track.release_hash(), AlbumArt::default());
                AlbumArt::default()
            }
            Err(e) => {
                eprintln!("Failed to get album art for \"{}\": {}", track.title, e);
                AlbumArt::default()
            }
        }
    }
}

#[cfg(not(feature = "musicbrainz"))]
impl AlbumArtProvider {
    fn fetch_album_art(&mut self, _track: &Track) -> AlbumArt {
        self.cache
            .insert(_track.release_hash(), AlbumArt::default());
        AlbumArt::default()
    }
}
