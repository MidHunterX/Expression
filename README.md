![banner](./.github/expression_banner.jpg)

# Expression

Expression is a highly customizable, time-aware wallpaper engine for Linux. It allows you to automate desktop wallpaper changes based on the current hour, randomly or evenly spread wallpapers from grouped directories, and define overrides for special times like sleep, lunch, or focus hours. Whether you want a subtle visual timetable or an expressive ambience that changes with your day, Expression makes your desktop emotionally intelligent.

> "This is the best 24-hour automatic wallpaper setter you could've ever asked for." - sis

## ⛲ Features

### Main Features

- [x] Supports multiple wallpaper setters (swww, feh)
- [x] 24-hour wallpaper cycling
- [x] Set specific wallpaper on specific hour
- [x] Set random wallpaper from a group of wallpapers for a specific hour
- [x] Distribute wallpapers from a group evenly across the hour
- [x] Override with special wallpaper based on a timetable (e.g., lunch, sleep)
- [x] Per group config overrides
- [x] Execute custom scripts on wallpaper change

## 🚀 Installation

Make sure you have a supported wallpaper setter (`swww`, `feh`) installed

### METHOD 1: Give me the EXE

#### Step 1: Download and Install

[Download](https://github.com/MidHunterX/Expression/releases/latest) the latest release.

#### Step 2: Configure

See [Configuration](#-configuration) section below.

### METHOD 2: Binaries? We don't do that here

`cargo` and `rustup` should be installed for building the project

#### Step 1: Build and Install

Clone the repository

```sh
git clone https://github.com/MidHunterX/Expression.git
cd Expression
```

Create config directory if it doesn't exist

```sh
mkdir -p ~/.config/expression
```

Compile and Install

```sh
cargo build --release
sudo cp target/release/expression /usr/local/bin/
```

#### Step 2: Configure

See [Configuration](#-configuration) section below.

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

Create a TOML configuration file at:

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
# Command to execute on wallpaper change
# Examples:
# execute_on_change = "~/.scripts/custom_script.sh"
execute_on_change = "notify-send 'Wallpaper Changed'"

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

## 🔥 Use Cases

### 24 hour cycle (Fixed Wallpaper)

Want a simple 24 hour timecycle wallpaper like this?

You can set up your wallpaper directory like this:

```c
00.jpg  03.jpg  06.jpg  09.jpg  12.jpg  15.jpg  18.jpg  21.jpg
01.jpg  04.jpg  07.jpg  10.jpg  13.jpg  16.jpg  19.jpg  22.jpg
02.jpg  05.jpg  08.jpg  11.jpg  14.jpg  17.jpg  20.jpg  23.jpg
```

`00.jpg` _item_ is used for 00 to 01 and `23.jpg` _item_ is used for 23 to 00

Note: numbers without preceeding 0 is valid too. For e.g.: `3.png`

### Don't like renaming?

Renaming wallpapers numerically can get a little tedious especially when you want to change it into a different time. There's a solution for that. Since directories are also a valid wallpaper _item_, we can use that to our advantage.

```sh
wallpapers/
├── 00/
│   └── Austrian Painter.png
├── 02/
│   └── Tiananmen Square.jpg
├── 03/
│   └── who_is_in_paris.jpg
...
```

Now you can freely move wallpapers between hours without worrying about filenames.

### A Visual Timetable (Wallpaper Override)

I'd like to get notified if its sleep time or its time for lunch via wallpaper. A truly non-intrusive way of communication. Since I work in a transparent terminal most of the time, the change is quite noticeable too. To do this:

1. Create a directory named `special` in your wallpaper directory
2. Put wallpapers for `sleep_time` and `lunch` _items_ in `special` directory

```sh
wallpaper_dir/
│   special/
│   ├── eating_ramyeon.jpg
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
13 = "eating_ramyeon"
23 = "sleep_time"
```

### Progressively Sleepy PC (Spread Wallpapers)

Okay the sleep_time wallpaper shows up at exactly 23:00 and then that's about it.

But what if your wallpapers could become progressively sleepier as the hour passes?

Let’s set that up.

1. Create a `sleep_time` directory inside the special directory, and add multiple progressively sleepy wallpapers:

```sh
wallpaper_dir/
│   special/
│   ├── eating_ramyeon.jpg
│   ├── sleep_time/
│   │   ├── sleepy_1.jpg
│   │   ├── sleepy_2.jpg
│   │   ├── ...
│   │   └── sleepy_10.jpg
...
```

2. Now let's configure that specific group to spread out throughout the sleep hour. Create a file named `config.toml` inside your group (e.g., `sleep_time/`) with the following content:

```toml
[general]
selection_strategy = "spread"  # Applies spread strategy to the group only
```

3. Make sure the name of the directory (e.g., `sleep_time`) is added to the `special_entries` section.

```toml
[special_entries]
13 = "eating_ramyeon"
23 = "sleep_time"
```

And just like that, your system gently drifts into dreamland with you 🌙

## 📜 License

This project is licensed under the **GNU General Public License v3.0** (GPL v3.0) - see the [LICENSE](LICENSE) file for details.

This ensures that all modifications and derivative works remain open-source and available to the community.
