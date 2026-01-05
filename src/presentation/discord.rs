use crate::{config::Config, state::PlaybackState, track::Track};
use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient, activity::Timestamps, error::Error as RpcError,
};

pub struct DiscordPresenter {
    client: DiscordIpcClient,
}

impl DiscordPresenter {
    pub fn new(client_id: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut client = DiscordIpcClient::new(client_id);
        client.connect()?;
        Ok(Self { client })
    }

    pub fn ensure_update(&mut self, state: &PlaybackState, config: &Config) {
        loop {
            match self.update(state, config) {
                Ok(_) => return,
                Err(e) => {
                    eprintln!("Discord RPC error: {e}. Attempting to reconnect...");

                    while let Err(e) = self.reconnect() {
                        eprintln!("Reconnect failed: {e}");
                        std::thread::sleep(std::time::Duration::from_secs(config.poll_idle));
                    }

                    println!("Reconnected!");
                }
            }
        }
    }

    pub fn update(&mut self, state: &PlaybackState, config: &Config) -> Result<(), RpcError> {
        use discord_rich_presence::activity::{Activity, ActivityType, Assets, StatusDisplayType};

        let track = match state {
            PlaybackState::Playing(track) if !config.excluded(track) => track,
            _ => return self.client.clear_activity(),
        };

        let mut payload = Activity::new()
            .status_display_type(StatusDisplayType::Details)
            .activity_type(ActivityType::Listening)
            .details(&track.title);

        if let Some(url) = &track.url {
            payload = payload.details_url(url)
        }

        let artists_str;
        if let Some(artists) = &track.artists {
            artists_str = artists.join(", ");
            payload = payload.state(&artists_str);
        }

        let small_image = config.get_player_image(&track.player);
        let assets = Assets::new()
            .large_image("icon")
            .small_image(&small_image)
            .small_text(&track.player);
        payload = payload.assets(assets);

        if let Some(timestamps) = Self::build_timestamps(track) {
            payload = payload.timestamps(timestamps);
        }

        self.client.set_activity(payload)
    }

    fn build_timestamps(track: &Track) -> Option<Timestamps> {
        use std::time::{SystemTime, UNIX_EPOCH};

        if let (Some(position), Some(length)) = (track.position, track.length)
            && !track.paused
        {
            let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
            let start_time = since_epoch.checked_sub(position)?;
            let end_time = start_time.checked_add(length)?;

            let timestamps = Timestamps::new()
                .start(start_time.as_secs().try_into().ok()?)
                .end(end_time.as_secs().try_into().ok()?);

            Some(timestamps)
        } else {
            None
        }
    }

    pub fn reconnect(&mut self) -> Result<(), RpcError> {
        self.client.reconnect()
    }
}
