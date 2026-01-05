use crate::track::Track;

#[cfg(feature = "mpris")]
pub mod mpris;

pub trait TrackSource {
    fn current_track(&self) -> Option<Track>;
}
