# Expression

## Overview

Expression is an algorithm based highly efficient, wallpaper selector designed
for power efficiency.

## Installation

### Prerequisites

- Rust (Latest stable version)
- A supported wallpaper setter (`swww`, `feh`)

### Build and Install

```sh
cargo build --release
cp target/release/expression ~/.local/bin/
```

## Usage

Run Expression as a daemon:

```sh
expression &
```

Run Expression with debug logs:

```sh
RUST_LOG=debug expression
```

### Configuration

Expression uses a TOML configuration file located at:

```sh
~/.config/expression/config.toml
```

#### Example Config

Options marked with `#` are optional.

```toml
[general]
backend = "swww"
# enable_special = true

[directories]
wallpaper = "~/Pictures/wallpaper_dir"
# special = "~/Pictures/Wallpapers/Special"

# [special_entries]
# 5 = "rise and shine"
# 9 = "workout_motivation"
# 23 = "sleep_time"
```

## Introduction && Features

It works by treating wallpapers as a single object; whether is is a file or
multiple files grouped into a directory.

### Wallpaper Objects

- **Wallpaper Entry** – A single wallpaper file named after the hour (e.g., `07.jpg`).
- **Wallpaper Group** – A folder containing multiple wallpapers, named after the hour (e.g., `07/`).

```
wallpaper_dir/
├── 00.jpg      # Entry for midnight
├── 05.jpg      # Entry for 5:00
├── 07/         # Group for 7:00
├── 21.jpg      # Entry for 21:00
├── 21/         # Group for 21:00
│   ├── E.jpg
│   ├── Zucc.jpg
│   └── ...
├── 22.jpg      # Entry for 22:00
└── ...
```

> Groups (dir/) have higher priority than Entries (file) by default.
> Currently when a Group is reached, a random wallpaper will be selected

### Collections

**Definition**: Directory with `non-numeric name` which contains Wallpaper Objects (Entry or Group).

By definition, `wallpaper_dir` itself is a Collection of Wallpapers as well.

```
wallpaper_dir/
│   special/      # Special Collection
│   ├── rise and shine.gif
│   ├── sleep_time.jpg
│   └── workout_motivation.jpg
│
│   collection_1/ # Custom Collection
│   ├── 00/
│   │   ├── Austrian Painter.png
│   │   ├── Tiananmen Square.jpg
│   │   ├── Who is in Paris.jpg
│   │   └── ...
│   ├── 01.gif
│   ├── 02.png
│   └── ...
│
│   Nature Collection by Twice/ # Custom Collection
│   └── ...
│
├── 00.jpg      # Entry for midnight
├── 05.jpg      # Entry for 5:00
├── 07/         # Group for 7:00
└── ...
```

> Wallpaper Objects in Special Collection has the highest priority over everything.

#### Special Collection

**Definition**: Collection dir with `special` as its name.

- By default `special` should be inside your wallpaper_dir
- You can change its location by configuring `special` in `[directories]`
- Special wallpaper entries are defined in `[special_entries]` section in the format of: `[hour] = "name"`
- Disable special collection by setting `enable_special = false` in `[general]`

```toml
[general]
backend = "swww"
enable_special = true

[directories]
wallpaper = "~/Pictures/Wallpapers/24_hour/"
special = "~/Pictures/Wallpapers/Special/"

[special_entries]
5 = "rise and shine"
9 = "workout_motivation"
23 = "sleep_time"
```

## Wallpaper Selection Strategies

Expression intelligently selects wallpapers based on the following strategies:

- **Fixed Time Strategy** – Assigns wallpapers to specific hours.
- **Spaced Out Time Strategy** – Evenly distributes wallpapers across time blocks.
- **Random Selection Strategy** – Chooses from available wallpapers at random.
- **Special Collection Strategy** – Overrides with high-priority special wallpapers.

## Refresh Strategy and Sleep Handling

- **T Strategy:** Waits until the next wallpaper refresh time. Most minimal cpu wake.
- **T/2 Strategy:** Recalculates refresh time every `T/2` seconds to mitigate sleep/hibernate issues.

## Roadmap

- **ACPI Wake Handling** – Detect system wake-up and recalculate the next refresh.
- **Smart Mood-Based Selection** – Dynamic wallpaper selection based on user context.
- **Integration with HunterOS** – Unified configuration with other HunterOS components.

## License

Expression is licensed under the **GNU Affero GPL v3**, ensuring all modifications and server-hosted versions remain open-source.

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.
