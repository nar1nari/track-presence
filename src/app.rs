use crate::{
    config::Config, presentation::discord::DiscordPresenter, sources::TrackSource,
    state::PlaybackState,
};

pub struct App {
    config: Config,
    sources: Box<[Box<dyn TrackSource>]>,
    playback_state: PlaybackState,
}

impl App {
    pub fn new(config: Config, sources: Box<[Box<dyn TrackSource>]>) -> Self {
        Self {
            config,
            sources,
            playback_state: PlaybackState::Stopped,
        }
    }

    pub fn run(&mut self) {
        println!("Connecting to Discord IPC...");
        let mut presenter = loop {
            match DiscordPresenter::new(&self.config.client_id) {
                Ok(p) => {
                    println!("Connected!");
                    break p;
                }
                Err(e) => {
                    eprintln!("Failed to connect: {e}. Retrying...");
                    std::thread::sleep(std::time::Duration::from_secs(self.config.poll_idle));
                }
            }
        };

        loop {
            let new_state = self
                .sources
                .iter()
                .map(|source| PlaybackState::derive_state(source.as_ref()))
                .find(|s| matches!(s, PlaybackState::Playing(_)))
                .unwrap_or(PlaybackState::Stopped);

            if new_state != self.playback_state {
                presenter.ensure_update(&new_state, &self.config);
                self.playback_state = new_state;
            }

            std::thread::sleep(std::time::Duration::from_secs(match self.playback_state {
                PlaybackState::Playing(_) => self.config.poll_playing,
                PlaybackState::Stopped => self.config.poll_idle,
            }));
        }
    }
}
