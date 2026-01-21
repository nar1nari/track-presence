use std::time::Duration;

use reqwest::blocking::Client;

use super::{AlbumArt, AlbumArtError, AlbumArtSource};
use crate::track::Track;

pub struct MusicBrainz {
    client: Client,
}

impl AlbumArtSource for MusicBrainz {
    fn get_album_art(&self, track: &Track) -> Result<Option<AlbumArt>, AlbumArtError> {
        let artists = match &track.artists {
            Some(a) => a,
            None => return Ok(None),
        };

        let release = track.album.as_deref().unwrap_or(&track.title);

        for artist in artists {
            if let Some(id) = self.find_release_id(release, artist)?
                && let Some(url) = self.get_album_art_url(&id)?
            {
                return Ok(Some(AlbumArt::from(url)));
            }
        }

        Ok(None)
    }
}

impl Default for MusicBrainz {
    fn default() -> Self {
        let client = Client::builder()
            .user_agent("track_presence/0.1")
            .connect_timeout(Duration::from_secs(3))
            .timeout(Duration::from_secs(5))
            .build()
            .expect("Failed to create HTTP client");
        Self { client }
    }
}

impl MusicBrainz {
    fn find_release_id(
        &self,
        release: &str,
        artist: &str,
    ) -> Result<Option<String>, AlbumArtError> {
        let query = format!("artist:{artist} AND release:{release}");
        let url = format!(
            "https://musicbrainz.org/ws/2/release-group/?query={}&limit=1",
            query
        );

        let body = self
            .client
            .get(&url)
            .send()
            .map_err(|_| AlbumArtError::Network)?
            .text()
            .map_err(|_| AlbumArtError::Network)?;

        let marker = r#"<release-group id=""#;

        let start = match body.find(marker) {
            Some(pos) => pos + marker.len(),
            None => return Ok(None),
        };

        let end = body[start..]
            .find('"')
            .ok_or(AlbumArtError::InvalidResponse)?
            + start;

        Ok(Some(body[start..end].to_string()))
    }

    fn get_album_art_url(&self, id: &str) -> Result<Option<String>, AlbumArtError> {
        let url = format!("https://coverartarchive.org/release-group/{id}/front-250");

        let resp = self
            .client
            .get(&url)
            .send()
            .map_err(|_| AlbumArtError::Network)?;

        if resp.status().is_success() {
            Ok(Some(url))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_existing_release_id() {
        let client = MusicBrainz::default();
        assert_eq!(
            client.find_release_id("Minecraft Volume Alpha", "C418"),
            Ok(Some(String::from("7f4856cd-f078-4c0d-aef7-48474d8fa890")))
        )
    }

    #[test]
    fn fail_to_find_release_id() {
        let client = MusicBrainz::default();
        assert_eq!(
            client.find_release_id("ofdjsaifojfjoda", "fjidosajj"),
            Ok(None)
        )
    }

    #[test]
    fn get_album_art_for_album() {
        let client = MusicBrainz::default();
        let id = "7f4856cd-f078-4c0d-aef7-48474d8fa890";
        assert_eq!(
            client.get_album_art_url(id),
            Ok(Some(format!(
                "https://coverartarchive.org/release-group/{id}/front-250"
            )))
        )
    }

    #[test]
    fn get_album_art_for_single() {
        let client = MusicBrainz::default();
        let id = "7510af18-6d4a-444d-8a52-2a438f0191a8";
        assert_eq!(
            client.get_album_art_url(id),
            Ok(Some(format!(
                "https://coverartarchive.org/release-group/{id}/front-250"
            )))
        )
    }

    #[test]
    fn fail_to_get_album_art() {
        let client = MusicBrainz::default();
        let id = "SomeID";
        assert_eq!(client.get_album_art_url(id), Ok(None))
    }
}
