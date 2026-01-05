![Cover](./assets/cover.png)

**Track Presence** is a lightweight Discord music status application that tracks what you’re listening to and displays it on Discord. It’s modular, fully local, and extremely resource-efficient — under 1 MB in size, minimal CPU, and memory usage.

# Features

- ✅ Lightweight, low CPU & RAM usage
- ✅ Fully local — no external connections
- ✅ Modular — pick only the music players you want
- ✅ Supports multiple players (currently via MPRIS)
- ✅ Exclusion options for players, artists, tracks, and URLs
- ✅ Optional custom Discord Client ID

# Installation

> **Linux only (for now).** Track Presence currently supports **MPRIS**. Support for other OSes and additional players will come in future updates.

Install with Cargo package manager:

```bash
cargo install track_presence
```

Run Track Presence:

```bash
trackpresence
```

To launch at system startup, add `trackpresence` to your autostart configuration.

# Usage

Track Presence runs entirely in the background — no GUI required.

**Optional Arguments**

- `--excluded-players` — hide certain players
- `--excluded-titles` — hide certain song titles
- `--excluded-artists` — hide certain artists
- `--excluded-urls` — hide songs from specific URLs

Example:

```bash
trackpresence --excluded-players mpv --excluded-urls "www.youtube.com,vk.com" --excluded-artists "Justin Bieber,Ironmouse"
```

Advanced users can use a custom Discord Client ID.

See all options with `trackpresence --help`.

# Contributing

We welcome contributions! Currently, the biggest need is adding support for more music players.

Steps to add a new player:

1. Create a module in [/src/sources/](./src/sources/) (e.g. [mpris.rs](./src/sources/mpris.rs))
2. Add a corresponding Cargo feature for the module to allow selection during compilation.
3. Add a module to the list in [/src/app.rs](./src/app.rs)

Your contributions will make Track Presence more versatile for everyone.

# License

[![Licence](https://img.shields.io/github/license/Ileriayo/markdown-badges?style=for-the-badge)](./LICENSE)
See [LICENSE](./LICENSE) for details.