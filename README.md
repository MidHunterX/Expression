![banner](./.github/expression_banner.jpg)

# Expression

_Time-based, rule-driven wallpaper automation for Wayland (and more!)_

## ⛲ Features

### Main Features

- [x] Supports multiple wallpaper setters (swww, feh)
- [x] 24-hour wallpaper cycling
- [x] Set specific wallpaper on specific hour
- [x] Set random wallpaper from a group of wallpapers for a specific hour
- [x] Distribute wallpapers from a group evenly across the hour
- [x] Override with special wallpaper based on a timetable (e.g., lunch, sleep)

### Technical Features

- [x] TOML-based configuration system
- [x] Background service/daemon support
- [x] Hierarchical wallpaper organization:
  - Root-level: Collections (e.g., Wallpapers, Catppuccin, Gruvbox, Not_SFW, Dark_Mode)
  - Mid-level: Time-based Wallpaper Items (Entries and Groups)
  - Leaf-level: Wallpapers (image files)

### Future Features

- [ ] Per group config overrides
- [ ] Interruptible Sleep Cycle
- [ ] Randomization Scopes:
  - [x] Within Current Group
  - [ ] Within Current Collection
  - [ ] Across all Collections (excluding Special)
  - [ ] Across all Wallpapers (excluding Special)
- [ ] Spread collection of wallpapers throughout the day
- [x] Spread group of wallpapers throughout an hour
- [ ] Efficient, event-driven execution (no polling or sleeps)
- [ ] Interactive Terminal User Interface (TUI) Configurator
- [ ] Inter Process Communication (IPC) support for external control or dynamic updates

## 🤷 Why?

This project began as a simple Bash script for timed wallpapers, but quickly became unmaintainable as feature requests grew. This Rust rewrite solves that with maintainability, performance, and scalability in mind.

## 🚀 Installation

### Prerequisites

- Rust (Latest stable version)
- A supported wallpaper setter (`swww`, `feh`)
- Configuration file (`~/.config/expression/config.toml`)

### Build and Install

```sh
# Clone the repository
git clone https://github.com/MidHunterX/Expression.git
cd Expression

# Build the project
cargo build --release

# Create config directory if it doesn't exist
mkdir -p ~/.config/expression

# Copy the binary to your local bin directory
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
# Way to select wallpaper from a group: random, spread
group_selection_strategy = "random"

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

Expression works by treating wallpapers as a single unit; whether it is a file
or multiple files grouped into a directory.

### Wallpaper Items

- **Entry** – A single wallpaper file named after the hour (e.g., `07.jpg`).
- **Group** – A directory containing multiple wallpapers, named after the hour (e.g., `07/`).

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

> Groups (directories) take precedence over Entries (individual files) by default.
> When a Group is active, a random wallpaper from within the group is selected by default.

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

## Use Cases & Creative Ideas

### ⌛ 24 hour cycle

Want a simple 24 hour timecycle wallpaper like this?

You can set up your wallpaper directory like this:

```c
00.jpg  03.jpg  06.jpg  09.jpg  12.jpg  15.jpg  18.jpg  21.jpg
01.jpg  04.jpg  07.jpg  10.jpg  13.jpg  16.jpg  19.jpg  22.jpg
02.jpg  05.jpg  08.jpg  11.jpg  14.jpg  17.jpg  20.jpg  23.jpg
```

`00.jpg` _item_ is used for 00 to 01 and `23.jpg` _item_ is used for 23 to 00

Note: numbers without preceeding 0 is valid too. For e.g.: `3.png`

### 📛 Don't like renaming?

Renaming wallpapers numerically can get a little tedious especially when you want to change it into a different time. There's a solution for that. Since directories are also a valid wallpaper _item_, we can use that to our advantage.

```sh
wallpapers/
├── 00/
│   └── Austrian Painter.png
├── 02/
│   ├── World Trade Center.jpg
│   └── Tiananmen Square.jpg
├── 03/
│   └── who_is_in_paris.jpg
...
```

Now you can freely move wallpapers between hours without worrying about filenames.

### 📌 Timetable Notification

I'd like to get notified if its sleep time or its time for lunch via wallpaper. A truly non-intrusive way of communication 😌. Since I work in a transparent terminal most of the time, the change is quite noticeable too. To do this:

1. Create a directory named `special` in your wallpaper directory
2. Put wallpapers for `sleep_time` and `lunch` _items_ in `special` directory

```sh
wallpaper_dir/
│   special/
│   ├── top_ramen.jpg
│   └── sleep_time.jpg
│
├── 00/
│   └── Austrian Painter.png
...
```

3. Now let's configure which time to run these special wallpaper overrides.

```toml
# Add the names of item (file/dir) along with the time to your config in special_entries
[special_entries]
13 = "lunch"
23 = "sleep_time"
```

### 😴 Progressively Sleepy PC

Okay the sleep_time wallpaper shows up at exactly 23:00 and then that's about it.

But what if your wallpapers could become progressively sleepier as the hour passes?

Let’s set that up.

1. Create a `sleep_time` directory inside the special directory, and add multiple progressively sleepy wallpapers:

```sh
wallpaper_dir/
│   special/
│   ├── top_ramen.jpg
│   ├── sleep_time/
│   │   ├── sleepy_1.jpg
│   │   ├── sleepy_2.jpg
│   │   ├── ...
│   │   └── sleepy_10.jpg
...
```

2. Now let's configure those to spread out throughout the sleep hour.

```toml
[general]
group_selection_strategy = "spread"  # Applies spread strategy to all directories

[special_entries]
13 = "lunch"
23 = "sleep_time"
```

And just like that, your system gently drifts into dreamland with you 🌙

> [!WARNING]
> group_selection_strategy can currently only be set globally in [general],
> so all directories will use spread until per-group support is added.

## 📜 License

Expression is licensed under the **GNU Affero GPL v3**, ensuring all modifications and server-hosted versions remain open-source.

## 🙌 Contributing

Contributions are welcome! Feel free to submit issues or pull requests.
