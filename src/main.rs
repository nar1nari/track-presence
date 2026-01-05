use clap::Parser;
use track_presence::{app, config::Config};

fn main() {
    let config = Config::parse();
    app::run(&config)
}
