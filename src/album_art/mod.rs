use std::collections::HashMap;

use thiserror::Error;

use crate::{
    track::{ReleaseHash, Track},
    utils::error_chain,
};

#[cfg(feature = "musicbrainz")]
pub mod music_brainz;

#[derive(Debug, Error)]
pub enum AlbumArtError {
    #[error("request to {url} failed: {source}")]
    Network {
        url: String,
        #[source]
        source: reqwest::Error,
    },
    #[error("invalid response from {url}: {reason}")]
    InvalidResponse { url: String, reason: String },
}

impl PartialEq for AlbumArtError {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
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

pub struct AlbumArtProvider {
    cache: HashMap<ReleaseHash, AlbumArt>,
    sources: Box<[Box<dyn AlbumArtSource>]>,
}

impl AlbumArtProvider {
    pub fn new(sources: Box<[Box<dyn AlbumArtSource>]>) -> Self {
        Self {
            cache: HashMap::new(),
            sources,
        }
    }

    pub fn get_album_art(&mut self, track: &Track) -> AlbumArt {
        let release_hash = track.release_hash();
        if let Some(art) = self.cache.get(&release_hash) {
            return art.clone();
        }
        let art = self.fetch_album_art(track);
        self.cache.insert(release_hash, art.clone());
        art
    }

    fn fetch_album_art(&self, track: &Track) -> AlbumArt {
        for source in self.sources.iter() {
            match source.get_album_art(track) {
                Ok(Some(art)) => return art,
                Ok(None) => continue,
                Err(e) => {
                    eprintln!(
                        "Failed to get album art for \"{}\": {}",
                        track.title,
                        error_chain(&e)
                    );
                    continue;
                }
            }
        }
        AlbumArt::default()
    }
}
