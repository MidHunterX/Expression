use chrono::{Local, Timelike};
use colored::Colorize;
use expression::backends::get_backend;
use expression::config::Config;
use expression::utils::{calc, logger, wallpaper};
use log2::{debug, error, info, warn};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let config = Config::load()?;

    let _log2 = logger::init();
    debug!("----------------------------------");

    // Backend Initialization
    let backend = get_backend(&config.general.backend)?;
    let backend_name = backend.name();
    let extensions = backend.supported_extensions();
    debug!(
        "Init Time ({}): {}",
        backend_name,
        format!("{:?}", start.elapsed()).blue()
    );

    // Config Variables
    let wallpaper_dir = config.directories.wallpaper.as_str();
    // Don't worry, JFK won't get executed here because defaults come from config
    let special_dir = config.directories.special.as_deref().unwrap_or(&"JFK");
    let config_special_entries = config.special_entries;
    let config_special_enabled = config.general.enable_special;
    let config_group_selection = config.general.group_selection_strategy;

    let mut selected_item = Vec::new();

    loop {
        let start = Instant::now();
        let now = Local::now();
        let hour = now.hour() as u8;
        debug!(
            "Current Time: {}",
            format!("{}", now.format("%H:%M")).cyan()
        );

        selected_item.clear();

        // TODO: Collection: Theme Override Strategy
        /* let collections = wallpaper::get_collections(wallpaper_dir)?;
        for collection in collections {
            println!("collection: {}", collection.display());
        } */

        // TODO: Randomized Scope Strategy

        // COLLECTION: Special Collection
        if selected_item.is_empty() && config_special_enabled {
            let special_items_result = wallpaper::get_special_items(special_dir, extensions);
            match special_items_result {
                Ok(special_items) => {
                    if let Some(filename) = config_special_entries.get(&hour.to_string()) {
                        if let Some(item) = special_items.get(filename) {
                            info!("Special Collection Activated!");
                            selected_item = wallpaper::select_wallpaper_item(item, extensions);
                        }
                    }
                }
                Err(err) => {
                    warn!("Special Collection Error: {}", err);
                }
            }
        }

        // COLLECTION: Normal Collection
        if selected_item.is_empty() {
            let items = wallpaper::get_wallpaper_items(wallpaper_dir, extensions, Some(hour))?;
            if let Some(item) = items.get(&hour) {
                selected_item = wallpaper::select_wallpaper_item(item, extensions);
            }
        }

        // Wallpaper Selection Strategies
        debug!("Exec Time: {}", format!("{:?}", start.elapsed()).cyan());
        let start = Instant::now();

        let mut interval = 60; // Minutes
        let mut refresh_strategy = "T2";

        let item_size = selected_item.len();
        if item_size == 0 {
            warn!("No wallpaper available for {}", hour);
        } else if item_size == 1 {
            // SELECT: Fixed Time Strategy
            backend.apply_wallpaper(&selected_item[0])?;
            info!("Wallpaper applied successfully!");
        } else if item_size > 1 {
            if config_group_selection == "spread" {
                // SELECT: Spread Strategy
                // Algorithm
                // - Wallpapers should be less than 60 (eg.5)
                // - refresh_interval = 60/len (eg. 60/5 = 12)
                // - wallpaper_index = ceil(minute_now/interval) (eg. 30/12 = 3)
                const MAX_SPREAD_ITEMS: usize = 60;
                if item_size > MAX_SPREAD_ITEMS {
                    warn!("Too many wallpapers to spread effectively ({item_size} > {MAX_SPREAD_ITEMS})");
                }
                let refresh_interval = (60/item_size) as u32;
                // ceil = (a + b - 1) / b
                let mut wallpaper_index = (now.minute() + refresh_interval - 1) / refresh_interval;
                wallpaper_index = wallpaper_index.min(item_size as u32); // Avoid overflow
                println!("wallpaper_index: [{}/{}]", wallpaper_index, item_size);
                backend.apply_wallpaper(&selected_item[wallpaper_index as usize])?;
                info!("Wallpaper(spread) applied successfully!");
                // Overrides
                interval = refresh_interval as u32;
                refresh_strategy = "T";
            } else {
                // SELECT: Random Strategy
                info!("Multiple wallpapers available for {}", hour);
                let wallpaper_index = rand::random_range(0..item_size);
                backend.apply_wallpaper(&selected_item[wallpaper_index])?;
                info!("Wallpaper(group) applied successfully!");
            }
        }

        debug!(
            "Exec Time ({}): {}",
            backend_name,
            format!("{:?}", start.elapsed()).blue()
        );

        // TODO: Wait Strategy:
        // (entries.len() / 24) for spaced out

        // Wait: 24 Hour Cycle Strategy
        let wait_seconds = calc::wait_time(interval, now);
        info!("Waiting for {}m {}s", wait_seconds / 60, wait_seconds % 60);

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
