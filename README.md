![banner](./.github/expression_banner.jpg)

# Expression

Expression is a lightweight and highly efficient time based wallpaper auto-selector for Wayland, optimized for minimal system resource usage. Designed with performance and customizability in mind, it selects wallpapers based on hour and user-defined rules.

## ⛲ Features

### Main Features

- [x] Supports multiple wallpaper setters (swww, feh)
- [x] 24-hour wallpaper cycling
- [x] Set specific wallpaper on specific hour
- [x] Set random wallpaper from a group of wallpapers for a specific hour
- [x] Override with special wallpaper based on a timetable (e.g., lunch, sleep)

### Technical Features

- [x] TOML-based configuration system
- [x] Background service/daemon support
- [x] Hierarchical wallpaper organization:
  - Root-level: Collections (e.g., Wallpapers, Catppuccin, Gruvbox, Not_SFW, Dark_Mode)
  - Mid-level: Time-based Wallpaper Items (Entries and Groups)
  - Leaf-level: Wallpapers (image files)

### Future Features

- [ ] Randomization Scopes:
  - [x] Current Group
  - [ ] Current Collection
  - [ ] All Collections (excluding Special)
  - [ ] All Wallpapers (excluding Special)
- [ ] Spread collection of wallpapers throughout the day
- [ ] Spread group of wallpapers throughout an hour
- [ ] Efficient, event-driven execution (no polling or sleeps)
- [ ] Terminal UI configurator
- [ ] Inter process communication (IPC) support

## 🤷 Why?

Expression was initially a simple bash script for timed wallpapers. However, it quickly became unmaintainable as I wanted to add more features. That is exactly what this project is: a fully maintainable, optimized timed wallpaper script with advanced features.

## 🚀 Installation

### Prerequisites

- Rust (Latest stable version)
- A supported wallpaper setter (`swww`, `feh`)
- Configuration file (`~/.config/expression/config.toml`)

### Build and Install

```sh
cargo build --release
cp target/release/expression ~/.local/bin/
```

## 👟 Usage

Run Expression as a daemon:

```sh
expression &
```

Run Expression with debug logs:

```sh
RUST_LOG=debug expression
```

## 🔧 Configuration

Expression uses a TOML configuration file located at:

```sh
~/.config/expression/config.toml
```

### Minimal Config

```toml
[general]
backend = "swww"

[directories]
wallpaper = "~/Pictures/wallpaper_dir"
```

### Full Config

```toml
[general]
# Supported backends: swww, feh
backend = "swww"
# Enable/Disable special collection feature
enable_special = true

[directories]
# Default wallpaper directory
wallpaper = "~/Pictures/wallpaper_dir"
# Override special wallpaper directory (default: wallpaper_dir/special)
special = "~/Pictures/Wallpapers/Special"

[special_entries]
# Wallpaper item (entry/group) names situated inside special collection along with their corresponding hour
# These special wallpaper items always take precedence over other wallpaper items
5 = "rise and shine"
9 = "workout_motivation"
23 = "sleep_time"
```

## 📚 Concepts

Expression works by treating wallpapers as a single unit; whether is is a file
or multiple files grouped into a directory.

### Wallpaper Items

- **Entry** – A single wallpaper file named after the hour (e.g., `07.jpg`).
- **Group** – A folder containing multiple wallpapers, named after the hour (e.g., `07/`).

```sh
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

**Definition**: Directory with `non-numeric name` which contains Wallpaper Items (Entry or Group).

By definition, `wallpaper_dir` itself is a Collection of Wallpapers as well.

```sh
wallpaper_dir/
│   special/                    # Special Collection
│   ├── rise and shine.gif
│   ├── sleep_time.jpg
│   └── workout_motivation.jpg
│
│   collection_1/               # Custom Collection 1
│   ├── 00/
│   │   ├── Austrian Painter.png
│   │   ├── Tiananmen Square.jpg
│   │   ├── who_is_in_paris.jpg
│   │   └── ...
│   ├── 01.gif
│   ├── 02.png
│   └── ...
│
│   Nature Collection by Twice/ # Custom Collection 2
│   └── ...
│
├── 00.jpg  # Entry for midnight
├── 05.jpg  # Entry for 5:00
├── 07/     # Group for 7:00
└── ...
```

> Wallpaper Objects in Special Collection has the highest priority over everything.

### Special Collection

**Definition**: Collection dir which has the highest priority when selecting wallpapers.

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

## 📜 License

Expression is licensed under the **GNU Affero GPL v3**, ensuring all modifications and server-hosted versions remain open-source.

## 🙌 Contributing

Contributions are welcome! Feel free to submit issues or pull requests.
