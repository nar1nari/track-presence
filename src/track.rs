#[derive(Debug, PartialEq)]
pub struct Track {
    pub player: String,
    pub title: String,
    pub url: Option<String>,
    pub album: Option<String>,
    pub artists: Option<Vec<String>>,
    pub position: Option<std::time::Duration>,
    pub length: Option<std::time::Duration>,
    pub paused: bool,
}

pub type ReleaseHash = (String, Vec<String>);

impl Track {
    pub fn release_hash(&self) -> ReleaseHash {
        let release_title = if let Some(album) = &self.album {
            album
        } else {
            &self.title
        };
        let release_artists = self.artists.clone().unwrap_or_default();
        (release_title.to_string(), release_artists)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_track() -> Track {
        Track {
            player: String::new(),
            title: String::from("Some track"),
            url: None,
            album: Some(String::from("Some album")),
            artists: Some(vec![String::from("Artist 1"), String::from("Artist 2")]),
            position: None,
            length: None,
            paused: false,
        }
    }

    #[test]
    fn release_hash_with_album() {
        let track = base_track();
        assert_eq!(
            track.release_hash(),
            (
                String::from("Some album"),
                vec![String::from("Artist 1"), String::from("Artist 2")]
            )
        );
    }

    #[test]
    fn release_hash_without_album() {
        let mut track = base_track();
        track.album = None;
        assert_eq!(
            track.release_hash(),
            (
                String::from("Some track"),
                vec![String::from("Artist 1"), String::from("Artist 2")]
            )
        );
    }

    #[test]
    fn release_hash_without_artist() {
        let mut track = base_track();
        track.artists = None;
        assert_eq!(track.release_hash(), (String::from("Some album"), vec![]));
    }
}
