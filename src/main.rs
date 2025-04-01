use chrono::{Local, Timelike};
use colored::Colorize;
use expression::backends::get_backend;
use expression::config::Config;
use expression::utils::{calc, wallpaper};
use log::{debug, error, info, warn};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let config = Config::load()?;

    let backend = get_backend(&config.general.backend)?;
    backend.initialize()?;
    let backend_name = backend.name();
    let extensions = backend.supported_extensions();

    env_logger::builder().format_timestamp(None).init();
    debug!(
        "Init Time ({}): {}",
        backend_name,
        format!("{:?}", start.elapsed()).blue()
    );

    let wallpaper_dir = config.directories.wallpaper.as_str();
    // Don't worry, JFK won't get executed here because defaults come from config
    let special_dir = config.directories.special.as_deref().unwrap_or(&"JFK");
    let special_entries_map = config.special_entries;
    let special_entries_enabled = config.general.enable_special;
    let groups_enabled = config.general.enable_groups;

    let mut selected_wallpaper = String::new();

    loop {
        let start = Instant::now();
        let now = Local::now();
        let hour = now.hour() as u8;
        debug!(
            "Current Time: {}",
            format!("{}", now.format("%H:%M")).cyan()
        );

        selected_wallpaper.clear();

        // TODO: Collection: Theme Override Strategy
        /* let collections = wallpaper::get_collections(wallpaper_dir)?;
        for collection in collections {
            println!("collection: {}", collection.display());
        } */

        // TODO: Randomized Scope Strategy

        // NOTE: Collection: Special Strategy
        if selected_wallpaper.is_empty() && special_entries_enabled {
            let special_entries = wallpaper::get_special_entries(special_dir, extensions);
            match special_entries {
                Ok(entries_map) => {
                    if let Some(filename) = special_entries_map.get(&hour.to_string()) {
                        if let Some(entry_vector) = entries_map.get(filename) {
                            for entry in entry_vector {
                                match entry {
                                    // SPECIAL GROUP
                                    wallpaper::WallpaperEntry::Directory(path) => {
                                        if !groups_enabled {
                                            continue;
                                        }
                                        // Selection: Random Strategy
                                        let sub_collection_dir = path.display().to_string();
                                        let sub_entries = wallpaper::get_wallpapers(
                                            &sub_collection_dir,
                                            extensions,
                                        )?;
                                        let wallpaper_index =
                                            rand::random_range(0..sub_entries.len());
                                        selected_wallpaper =
                                            sub_entries[wallpaper_index].display().to_string();
                                        info!(
                                            "Selected Special: [{}/{}] {}",
                                            wallpaper_index,
                                            sub_entries.len(),
                                            selected_wallpaper.split('/').last().unwrap()
                                        );
                                        break;
                                    }
                                    // SPECIAL ENTRY
                                    wallpaper::WallpaperEntry::File(path) => {
                                        // Selection: Fixed Time Strategy
                                        selected_wallpaper = path.display().to_string();
                                        info!(
                                            "Selected Special: {}",
                                            selected_wallpaper.split('/').last().unwrap()
                                        );
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    warn!("Special Collection Error: {}", err);
                }
            }
        }

        // NOTE: Collection: Fixed Time Strategy
        if selected_wallpaper.is_empty() {
            let entries_map =
                wallpaper::get_wallpaper_entries(wallpaper_dir, extensions, Some(hour))?;
            if let Some(entry_vector) = entries_map.get(&hour) {
                for entry in entry_vector {
                    match entry {
                        // WALLPAPER GROUP
                        wallpaper::WallpaperEntry::Directory(path) => {
                            if !groups_enabled {
                                continue;
                            }
                            // Selection: Random Strategy
                            let sub_collection_dir = path.display().to_string();
                            let sub_entries =
                                wallpaper::get_wallpapers(&sub_collection_dir, extensions)?;
                            let wallpaper_index = rand::random_range(0..sub_entries.len());
                            selected_wallpaper = sub_entries[wallpaper_index].display().to_string();
                            info!(
                                "Selected Wallpaper: [{}/{}] {}",
                                wallpaper_index,
                                sub_entries.len(),
                                selected_wallpaper.split('/').last().unwrap()
                            );
                            break;
                        }
                        // WALLPAPER ENTRY
                        wallpaper::WallpaperEntry::File(path) => {
                            // Selection: Fixed Time Strategy
                            selected_wallpaper = path.display().to_string();
                            info!(
                                "Selected Wallpaper: {}",
                                selected_wallpaper.split('/').last().unwrap()
                            );
                            break;
                        }
                    }
                }
            }
        }

        if selected_wallpaper.is_empty() {
            warn!("No wallpaper selected");
        }

        // TODO: Wallpaper Selection Strategies
        // Spaced Out Time Selection Strategy
        // Random Selection Strategy

        debug!("Exec Time: {}", format!("{:?}", start.elapsed()).cyan());
        let start = Instant::now();

        if selected_wallpaper.is_empty() {
            warn!("No wallpaper available for {}", hour);
        } else {
            backend.apply_wallpaper(&selected_wallpaper)?;
            info!("Wallpaper applied successfully!");
        }

        debug!(
            "Exec Time ({}): {}",
            backend_name,
            format!("{:?}", start.elapsed()).blue()
        );

        // TODO: Wait Strategy:
        // (entries.len() / 24) for spaced out

        // Wait: 24 Hour Cycle Strategy
        let interval = 60; // Minutes
        let wait_seconds = calc::wait_time(interval, now);
        info!("Waiting for {}m {}s", wait_seconds / 60, wait_seconds % 60);

        let refresh_strategy = "T2";
        if refresh_strategy == "T2" {
            calc::refresh_t2(interval, now, wait_seconds);
        } else if refresh_strategy == "T" {
            calc::sleep(wait_seconds);
        } else {
            error!("Invalid Refresh Strategy: {}", refresh_strategy);
            break; // Breaks out of main loop and exits to avoid infinite loop
        }
    }
    Ok(())
}
