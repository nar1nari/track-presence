use crate::{sources::TrackSource, track::Track};

#[derive(Debug, PartialEq)]
pub enum PlaybackState {
    Playing(Track),
    Stopped,
}

impl PlaybackState {
    pub fn derive_state(source: &(impl TrackSource + ?Sized)) -> PlaybackState {
        match source.current_track() {
            Some(track) => PlaybackState::Playing(track),
            None => PlaybackState::Stopped,
        }
    }
}
