use crate::{
    config::Config,
    presentation::discord::DiscordPresenter,
    sources::{self, TrackSource},
    state::PlaybackState,
};

pub fn run(config: &Config) {
    let sources: Vec<Box<dyn TrackSource>> = vec![
        #[cfg(feature = "mpris")]
        Box::new(sources::mpris::MprisSource),
    ];
    let mut state = PlaybackState::Stopped;

    println!("Connecting to Discord IPC...");
    let mut presenter = loop {
        match DiscordPresenter::new(&config.client_id) {
            Ok(p) => {
                println!("Connected!");
                break p;
            }
            Err(e) => {
                eprintln!("Failed to connect: {e}. Retrying...");
                std::thread::sleep(std::time::Duration::from_secs(config.poll_idle));
            }
        }
    };

    loop {
        let new_state = sources
            .iter()
            .map(|source| PlaybackState::derive_state(source.as_ref()))
            .find(|s| matches!(s, PlaybackState::Playing(_)))
            .unwrap_or(PlaybackState::Stopped);

        if new_state != state {
            presenter.ensure_update(&new_state, config);
            state = new_state;
        }

        std::thread::sleep(std::time::Duration::from_secs(match state {
            PlaybackState::Playing(_) => config.poll_playing,
            PlaybackState::Stopped => config.poll_idle,
        }));
    }
}
