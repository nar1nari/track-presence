use clap::Parser;
#[allow(unused_imports)]
use track_presence::{
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

    let mut app = App::new(config, sources.into());
    app.run()
}
