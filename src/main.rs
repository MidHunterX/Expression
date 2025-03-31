use chrono::{Local, Timelike};
use expression::backends::get_backend;
use expression::config::Config;
use expression::utils::wallpaper;
use log::{debug, error, info, warn};
use std::{
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let config = Config::load()?;

    let backend = get_backend(&config.general.backend)?;
    backend.initialize()?;
    let backend_name = backend.name();
    let extensions = backend.supported_extensions();

    env_logger::builder().format_timestamp(None).init();
    debug!("Init Time ({}): {:?}", backend_name, start.elapsed());

    let wallpaper_dir = config.directories.wallpaper.as_str();
    // Don't worry, JFK doesn't get Executed as defaults come from config
    let special_dir = config.directories.special.as_deref().unwrap_or(&"JFK");
    let special_entries_map = config.special_entries;
    let special_entries_enabled = config.general.enable_special;
    let sub_collection_enabled = config.general.enable_sub_collection;

    let mut selected_wallpaper = String::new();

    loop {
        let start = Instant::now();
        let now = Local::now();
        let seconds = now.second();
        let minute = now.minute();
        let hour = now.hour() as u8;
        debug!("This Hour: {}", hour);

        selected_wallpaper.clear();

        // TODO: Collection: Theme Override Strategy
        let collections = wallpaper::get_collections(wallpaper_dir)?;
        for collection in collections {
            println!("collection: {}", collection.display());
        }

        // TODO: Randomized Scope Strategy

        // NOTE: Collection: Special Strategy
        if selected_wallpaper.is_empty() && special_entries_enabled {
            let special_entries = wallpaper::get_wallpapers(special_dir, extensions);
            match special_entries {
                Ok(entries) => {
                    if let Some(filename) = special_entries_map.get(&hour.to_string()) {
                        if let Some(matching_path) = entries.iter().find(|path| {
                            path.file_stem()
                                .and_then(|s| s.to_str())
                                .map(|s| s == filename)
                                .unwrap_or(false)
                        }) {
                            info!(
                                "Special Wallpaper for {}: {}",
                                hour,
                                matching_path
                                    .file_name()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("Unknown")
                            );
                            selected_wallpaper = matching_path.display().to_string();
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
                        // SUB-COLLECTION
                        wallpaper::WallpaperEntry::Directory(path) => {
                            if !sub_collection_enabled {
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
                        // ENTRY
                        wallpaper::WallpaperEntry::File(path) => {
                            // Selection: Fixed Time Strategy
                            selected_wallpaper = path.display().to_string();
                            info!(
                                "Selected wallpaper: {}",
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

        debug!("Exec Time: {:?}", start.elapsed());
        let start = Instant::now();

        if selected_wallpaper.is_empty() {
            warn!("No wallpaper available for {}", hour);
        } else {
            backend.apply_wallpaper(&selected_wallpaper)?;
            info!("Wallpaper for {} Applied Successfully", hour);
        }

        let duration = start.elapsed();
        debug!("Exec Time ({}): {:?}", backend_name, duration);

        // TODO: Wait Strategy:
        // (entries.len() / 24) for spaced out

        // Wait: 24 Hour Cycle Strategy
        let refresh_time = 60;
        let remaining_seconds = 60 - seconds;
        let wait_minutes = refresh_time - (minute % refresh_time) - 1; // -1 for calculating current remaining_seconds
        let wait_seconds: u64 = ((wait_minutes * 60) + remaining_seconds).into();
        info!(
            "Waiting for {} minutes and {} seconds",
            wait_minutes, remaining_seconds
        );

        // TODO: Move this to config
        let refresh_strategy = "T/2";

        // Refresh: T/2 Strategy
        // Re-calculates refresh time every T/2 seconds
        // Mitigates the Sleep/Hibernate issue to an extent without much wakeup calls
        if refresh_strategy == "T/2" {
            let mut refresh_seconds = wait_seconds;
            while refresh_seconds > 1 {
                refresh_seconds /= 2;

                // Clamp refresh_seconds
                if refresh_seconds < 2 {
                    refresh_seconds = 2;
                }

                debug!(
                    "Refreshing again in {}...",
                    if refresh_seconds > 60 {
                        format!("{} minutes", refresh_seconds / 60)
                    } else {
                        format!("{} seconds", refresh_seconds)
                    }
                );

                thread::sleep(Duration::from_secs(refresh_seconds));

                // Re-calculate refresh time
                let now = Local::now();
                let seconds = now.second();
                let minute = now.minute();
                let new_hour = now.hour() as u8;
                // new_hour > hour = next hour
                // new_hour < hour = next day (midnight)
                if new_hour != hour {
                    debug!("Hour Changed: {}", new_hour);
                    break;
                }
                let remaining_seconds = 60 - seconds;
                let wait_minutes = refresh_time - (minute % refresh_time) - 1;
                let new_wait_seconds: u64 = ((wait_minutes * 60) + remaining_seconds).into();
                refresh_seconds = new_wait_seconds;
                // Edge Case - would never happen as loop already breaks on next hour
                // new_wait_seconds would almost be similar to refresh_seconds
                // because new_wait_seconds = prev refresh_seconds / 2
                /* if new_wait_seconds < refresh_seconds {
                    refresh_seconds = new_wait_seconds;
                } */
            }
            debug!("Out of T/2 Refresh Loop");
            thread::sleep(Duration::from_secs(1));
        }
        // Refresh: T Strategy
        // Simply waits until next wallpaper refresh time
        else if refresh_strategy == "T" {
            thread::sleep(Duration::from_secs(wait_seconds));
        } else {
            error!("Invalid Refresh Strategy: {}", refresh_strategy);
            break; // Breaks out of main loop and exits to avoid infinite loop
        }
    }
    Ok(())
}
