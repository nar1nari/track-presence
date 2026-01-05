use super::TrackSource;
use crate::track::Track;

pub struct MprisSource;

impl TrackSource for MprisSource {
    fn current_track(&self) -> Option<Track> {
        let player_finder = mpris::PlayerFinder::new().ok()?;
        let player = player_finder.find_active().ok()?;
        let metadata = player.get_metadata().ok()?;
        let title = metadata.title()?.to_string();
        let paused = if let Ok(status) = player.get_playback_status() {
            !matches!(status, mpris::PlaybackStatus::Playing)
        } else {
            false
        };

        Some(Track {
            player: player.identity().to_string(),
            title,
            url: metadata.url().map(|s| s.to_string()),
            artists: metadata
                .artists()
                .map(|v| v.iter().map(|s| s.to_string()).collect()),
            position: player.get_position().ok(),
            length: metadata.length(),
            paused,
        })
    }
}
