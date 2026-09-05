use clap::Parser;
use trackpresence::album_art::{self, AlbumArtSource};
#[allow(unused_imports)]
use trackpresence::{
    app::App,
    config::Config,
    sources::{self, TrackSource},
};

fn main() {
    let config = Config::parse();
    let sources: Vec<Box<dyn TrackSource>> = vec![
        #[cfg(feature = "mpris")]
        Box::new(sources::mpris::MprisSource),
        #[cfg(feature = "windows_media")]
        Box::new(sources::windows_media::WindowsMediaSource),
    ];

    let album_art_sources: Vec<Box<dyn AlbumArtSource>> = vec![
        #[cfg(feature = "musicbrainz")]
        Box::new(album_art::music_brainz::MusicBrainz::default()),
    ];

    let mut app = App::new(config, sources.into(), album_art_sources.into());
    app.run()
}
