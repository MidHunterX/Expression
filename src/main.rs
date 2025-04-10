use chrono::{Local, Timelike};
use colored::Colorize;
use expression::backends::get_backend;
use expression::config::Config;
use expression::utils::{calc, wallpaper};
use log2::{debug, error, info, warn};
use std::{path::PathBuf, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let config = Config::load()?;

    let backend = get_backend(&config.general.backend)?;
    backend.initialize()?;
    let backend_name = backend.name();
    let extensions = backend.supported_extensions();

    // Log: File + Stdout Debug Logs
    let logfile = dirs::state_dir()
        .map(|path| path.join("expression/expression.log"))
        .unwrap_or_else(|| PathBuf::from(""));
    // let _log2 = log2::open(logfile.to_str().unwrap()).tee(true).start();
    let _log2 = log2::open(logfile.to_str().unwrap()).start();

    // Log: Stdout Debug Logs
    // env_logger::builder().format_timestamp(None).init();

    debug!("----------------------------------");
    debug!(
        "Init Time ({}): {}",
        backend_name,
        format!("{:?}", start.elapsed()).blue()
    );

    let wallpaper_dir = config.directories.wallpaper.as_str();
    // Don't worry, JFK won't get executed here because defaults come from config
    let special_dir = config.directories.special.as_deref().unwrap_or(&"JFK");
    let config_special_entries = config.special_entries;
    let config_special_enabled = config.general.enable_special;

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

        // COLLECTION: Special Collection
        if selected_wallpaper.is_empty() && config_special_enabled {
            let special_items_result = wallpaper::get_special_items(special_dir, extensions);
            match special_items_result {
                Ok(special_items) => {
                    if let Some(filename) = config_special_entries.get(&hour.to_string()) {
                        if let Some(item) = special_items.get(filename) {
                            info!("Special Collection Activated!");
                            selected_wallpaper = wallpaper::select_wallpaper(item, extensions);
                        }
                    }
                }
                Err(err) => {
                    warn!("Special Collection Error: {}", err);
                }
            }
        }

        // COLLECTION: Normal Collection
        if selected_wallpaper.is_empty() {
            let items = wallpaper::get_wallpaper_items(wallpaper_dir, extensions, Some(hour))?;
            if let Some(item) = items.get(&hour) {
                selected_wallpaper = wallpaper::select_wallpaper(item, extensions);
            }
        }

        if selected_wallpaper.is_empty() {
            warn!("No wallpaper selected");
        }

        // TODO: Wallpaper Selection Strategies
        // Spaced Out Time Selection Strategy
        // - to do this, return Vec of wallpapers instead of String
        // - if Vec.len() > 1, space out
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
