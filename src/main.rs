use clap::Parser;
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
    ];

    let mut app = App::new(config, sources.into());
    app.run()
}
