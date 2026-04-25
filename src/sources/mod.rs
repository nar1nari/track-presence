use crate::track::Track;

#[cfg(feature = "mpris")]
pub mod mpris;

#[cfg(feature = "windows_media")]
pub mod windows_media;

pub trait TrackSource {
    fn current_track(&self) -> Option<Track>;
}
