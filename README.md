![Cover](./assets/cover.png)

**Track Presence** is a lightweight Discord music status application that tracks what you’re listening to and displays it on Discord. It’s modular, fully local, and extremely resource-efficient — under 1 MB in size, minimal CPU, and memory usage.

# Features

- ✅ Lightweight, low CPU & RAM usage
- ✅ Fully local — no external connections
- ✅ Modular — pick only the music players you want
- ✅ Supports multiple players
- ✅ Exclusion options for players, artists, tracks, and URLs
- ✅ Optional custom Discord Client ID
- ✅ Album art support via MusicBrainz

# Installation

Install with Cargo package manager:

```bash
cargo install track_presence --features {mpris/windows_media}
```

Run Track Presence:

```bash
trackpresence
```

To launch at system startup, add `trackpresence` to your autostart configuration.

# Cargo features

| Feature         | Description                                | Default  |
| --------------- | ------------------------------------------ | -------- |
| `mpris`         | Enables MPRIS player support               | ❌ No    |
| `windows_media` | Enables Windows Media player support       | ❌ No    |
| `musicbrainz`   | Enables album art fetching via MusicBrainz | ✅ Yes   |

If you don't want to connect to MusicBrainz to fetch album art, you can disable this feature accordingly:

```bash
cargo install track_presence --no-default-features --features mpris
```

But note that album art will not appear in rich presence.

# Usage

Track Presence runs entirely in the background — no GUI required.

**Optional Arguments**

- `--excluded-players` — hide certain players
- `--excluded-titles` — hide certain song titles
- `--excluded-artists` — hide certain artists
- `--excluded-urls` — hide songs from specific URLs (works only with MPRIS)

Example:

```bash
trackpresence --excluded-players mpv --excluded-urls "www.youtube.com,vk.com" --excluded-artists "Justin Bieber,Ironmouse"
```

Advanced users can use a custom Discord Client ID.

See all options with `trackpresence --help`.

# Future Plans

- MPD player support
- MacOS support

# License

[![Licence](https://img.shields.io/github/license/Ileriayo/markdown-badges?style=for-the-badge)](./LICENSE)

See [LICENSE](./LICENSE) for details.
