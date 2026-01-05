use clap::Parser;

use crate::track::Track;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Poll interval (in seconds) while media is playing
    #[arg(long, default_value_t = 1)]
    pub poll_playing: u64,

    /// Poll interval (in seconds) while no media is playing
    #[arg(long, default_value_t = 10)]
    pub poll_idle: u64,

    /// Comma-separated list of excluded media players
    ///
    /// Media from these players will not appear in your activity.
    #[arg(long, value_delimiter = ',')]
    pub excluded_players: Vec<String>,

    /// Exact (case-sensitive) comma-separated list of excluded media titles
    ///
    /// Media with these titles will not appear in your activity.
    #[arg(long, value_delimiter = ',')]
    pub excluded_titles: Vec<String>,

    /// Exact (case-sensitive) comma-separated list of excluded artists
    ///
    /// Media by these artists will not appear in your activity.
    #[arg(long, value_delimiter = ',')]
    pub excluded_artists: Vec<String>,

    /// Comma-separated list of excluded media URLs
    ///
    /// Media with these URLs will not show up in your activity.
    #[arg(long, value_delimiter = ',')]
    pub excluded_urls: Vec<String>,

    /// Discord Application Client ID
    ///
    /// Defaults to the official application ID.
    #[arg(long, default_value_t = String::from("1457412556753994033"))]
    pub client_id: String,

    /// Comma-separated list of media players with supported icons
    ///
    /// Only use this when a custom Client ID is specified.
    #[arg(
        long,
        value_delimiter = ',',
        default_value = "Mozilla firefox,chromium,mpv,VLC media player,Spotify"
    )]
    pub known_players: Vec<String>,
}

impl Config {
    pub fn get_player_image(&self, player: &str) -> String {
        let player = player.to_lowercase().replace(' ', "");

        if self
            .known_players
            .iter()
            .any(|p| p.to_lowercase().replace(' ', "") == player)
        {
            player
        } else {
            "icon".to_string()
        }
    }

    pub fn excluded(&self, track: &Track) -> bool {
        let normalize = |s: &str| s.to_lowercase().replace(' ', "");

        let player = normalize(&track.player);
        if self.excluded_players.iter().any(|p| normalize(p) == player) {
            return true;
        }

        if self.excluded_titles.contains(&track.title) {
            return true;
        }

        if track.artists.as_ref().is_some_and(|artists| {
            artists
                .iter()
                .any(|artist| self.excluded_artists.contains(artist))
        }) {
            return true;
        }

        if track.url.as_ref().is_some_and(|url| {
            self.excluded_urls
                .iter()
                .any(|u| url.to_lowercase().contains(&u.to_lowercase()))
        }) {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::track::Track;

    fn base_config() -> Config {
        Config {
            poll_playing: 1,
            poll_idle: 10,
            client_id: String::new(),
            known_players: vec![],
            excluded_players: vec![],
            excluded_titles: vec![],
            excluded_artists: vec![],
            excluded_urls: vec![],
        }
    }

    fn base_track() -> Track {
        Track {
            player: String::new(),
            title: String::new(),
            url: None,
            artists: None,
            position: None,
            length: None,
            paused: false,
        }
    }

    #[test]
    fn player_image_known_player() {
        let mut cfg = base_config();
        cfg.known_players = vec!["mozilla firefox".into(), "Mpv".into()];
        assert_eq!(cfg.get_player_image("Mozilla Firefox"), "mozillafirefox");
        assert_eq!(cfg.get_player_image("mpv"), "mpv");
    }

    #[test]
    fn player_image_unknown_player_falls_back() {
        let mut cfg = base_config();
        cfg.known_players = vec!["mozilla firefox".into(), "Mpv".into()];
        assert_eq!(cfg.get_player_image("Some Random Player"), "icon");
    }

    #[test]
    fn excluded_by_player() {
        let mut cfg = base_config();
        let mut track = base_track();
        cfg.excluded_players = vec!["vlc".into(), "Spotify".into()];

        track.player = "spotify".into();
        assert!(cfg.excluded(&track));

        track.player = "mpv".into();
        assert!(!cfg.excluded(&track));
    }

    #[test]
    fn excluded_by_title() {
        let mut cfg = base_config();
        let mut track = base_track();
        cfg.excluded_titles = vec!["Bad Song".into()];

        track.title = "Bad Song".into();
        assert!(cfg.excluded(&track));

        track.title = "bad song".into();
        assert!(!cfg.excluded(&track));
    }

    #[test]
    fn excluded_by_artist() {
        let mut cfg = base_config();
        let mut track = base_track();
        cfg.excluded_artists = vec!["Annoying Artist".into()];

        track.artists = Some(vec!["Nice Artist".into(), "Annoying Artist".into()]);
        assert!(cfg.excluded(&track));

        track.artists = Some(vec!["Nice Artist".into(), "annoying artist".into()]);
        assert!(!cfg.excluded(&track));
    }

    #[test]
    fn excluded_by_url() {
        let mut cfg = base_config();
        let mut track = base_track();
        cfg.excluded_urls = vec!["www.youtube.com".into(), "example.com".into()];

        track.url = Some("https://EXAMPLE.com/watch?v=123".into());
        assert!(cfg.excluded(&track));

        track.url = Some("https://soundcloud.com/".into());
        assert!(!cfg.excluded(&track));
    }

    #[test]
    fn missing_optional_fields_do_not_exclude() {
        let cfg = base_config();
        let track = base_track();

        assert!(!cfg.excluded(&track));
    }
}
