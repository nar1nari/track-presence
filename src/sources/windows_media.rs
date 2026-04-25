use super::TrackSource;
use crate::track::Track;

pub struct WindowsMediaSource;

fn resolve_aumid_to_name(aumid: &str) -> String {
    use winreg::{RegKey, enums::*};

    let paths = [
        (
            HKEY_CURRENT_USER,
            format!("Software\\Classes\\AppUserModelId\\{aumid}"),
        ),
        (
            HKEY_LOCAL_MACHINE,
            format!("Software\\Classes\\AppUserModelId\\{aumid}"),
        ),
    ];

    for (hive, path) in &paths {
        let hive_key = RegKey::predef(*hive);
        if let Ok(key) = hive_key.open_subkey(path) {
            if let Ok(name) = key.get_value::<String, _>("DisplayName") {
                let resolved = expand_resource_string(&name).unwrap_or(name);
                if !resolved.is_empty() {
                    return resolved;
                }
            }
        }
    }

    if let Some(app_part) = aumid.split('!').last() {
        if app_part != aumid && !app_part.is_empty() {
            return app_part.to_string();
        }
    }

    aumid.to_string()
}

fn expand_resource_string(s: &str) -> Option<String> {
    if !s.starts_with('@') {
        return Some(s.to_string());
    }

    use windows::{Win32::UI::Shell::SHLoadIndirectString, core::PCWSTR};

    let wide: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
    let mut buf = vec![0u16; 256];

    unsafe {
        SHLoadIndirectString(PCWSTR(wide.as_ptr()), &mut buf, Some(std::ptr::null_mut())).ok()?;
    }

    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    Some(String::from_utf16_lossy(&buf[..end]))
}

impl TrackSource for WindowsMediaSource {
    fn current_track(&self) -> Option<Track> {
        use windows::Media::Control::{
            GlobalSystemMediaTransportControlsSessionManager,
            GlobalSystemMediaTransportControlsSessionPlaybackStatus,
        };

        let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
            .ok()?
            .get()
            .ok()?;

        let session = manager.GetCurrentSession().ok()?;

        let props = session.TryGetMediaPropertiesAsync().ok()?.get().ok()?;

        let title = props.Title().ok()?.to_string();
        if title.is_empty() {
            return None;
        }

        let paused = session
            .GetPlaybackInfo()
            .ok()
            .and_then(|info| info.PlaybackStatus().ok())
            .map(|status| {
                status != GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing
            })
            .unwrap_or(false);

        let (position, length) = session
            .GetTimelineProperties()
            .ok()
            .map(|tl| {
                let pos = tl.Position().ok().map(|d| {
                    std::time::Duration::from_nanos((d.Duration as u64).saturating_mul(100))
                });
                let len = tl.EndTime().ok().map(|d| {
                    std::time::Duration::from_nanos((d.Duration as u64).saturating_mul(100))
                });
                (pos, len)
            })
            .unwrap_or((None, None));

        let artist_str = props.Artist().ok().map(|s| s.to_string());
        let artists: Option<Vec<String>> = artist_str.filter(|s| !s.is_empty()).map(|s| vec![s]);

        let player = session
            .SourceAppUserModelId()
            .ok()
            .map(|s| resolve_aumid_to_name(&s.to_string()))
            .unwrap_or_else(|| "Track Presence".to_string());

        Some(Track {
            player,
            title,
            url: None,
            album: props
                .AlbumTitle()
                .ok()
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty()),
            artists,
            position,
            length,
            paused,
        })
    }
}
