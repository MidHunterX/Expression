use chrono::{Local, Timelike};
use colored::Colorize;
use ctrlc;
use expression::{
    backends::get_backend,
    config::{get_group_config, Config, GroupSelectionStrategy},
    core::wallpaper,
    utils::{calc, cmd, logger},
};
use log2::{debug, error, info, warn};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

const DEFAULT_INTERVAL_MINUTES: f64 = 60.0;
const SECONDS_PER_HOUR: u64 = 3600;
const SECONDS_PER_MINUTE: u64 = 60;

pub enum WaitStrategy {
    Sleep,   // Sleeps once for the entire interval
    Refresh, // Sleeps and recalculates multiple times
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let config = Config::load()?;

    let _log2 = logger::init();
    debug!("----------------------------------");

    // SETUP: Backend
    let backend = get_backend(&config.general.backend)?;
    let backend_name = backend.name();
    let extensions = backend.supported_extensions();
    debug!(
        "Init Time ({}): {}",
        backend_name,
        format!("{:?}", start.elapsed()).blue()
    );

    // SETUP: Config Variables
    let wallpaper_dir = config.directories.wallpaper.as_str();
    let mut special_dir = String::new();
    match config.directories.special {
        Some(dir) => special_dir = dir.as_str().to_string(),
        None => (), // Defaults are set in config initialization
    }
    let config_special_entries = config.special_entries;
    let config_special_enabled = config.general.enable_special;
    let config_group_strategy = config.general.group_selection_strategy;
    let exec_cmd = config.general.execute_on_change;

    let mut selected_item = Vec::new();

    // SETUP: Signal Handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!(": Ctrl+C detected. Bye 👋");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // MAIN: Main Loop
    while running.load(Ordering::SeqCst) {
        let start = Instant::now();
        let now = Local::now();
        let hour = now.hour() as u8;
        info!(
            "Current Time: {}",
            format!("{}", now.format("%H:%M:%S")).bright_purple()
        );

        let mut current_strategy = config_group_strategy.clone();

        // █▀▀ █▀█ █░░ █░░ █▀▀ █▀▀ ▀█▀ █ █▀█ █▄░█
        // █▄▄ █▄█ █▄▄ █▄▄ ██▄ █▄▄ ░█░ █ █▄█ █░▀█
        // Capture Current Collection Content

        // TODO: Collection: Theme Override Strategy
        /* let collections = wallpaper::get_collections(wallpaper_dir)?;
        for collection in collections {
            println!("collection: {}", collection.display());
        } */

        // TODO: Randomized Scope Strategy

        // COLLECTION: Special Collection
        if selected_item.is_empty() && config_special_enabled {
            match wallpaper::get_special_items(&special_dir, extensions) {
                Ok(special_items) => {
                    if let Some(filename) = config_special_entries.get(&hour.to_string()) {
                        if let Some(item) = special_items.get(filename) {
                            selected_item = wallpaper::select_wallpaper_item(item, extensions);
                            info!("Special Collection Activated!");

                            // Local Group Config Overrides
                            if let Some(local_config) = get_group_config(item) {
                                if let Some(local_general) = local_config.general {
                                    if let Some(local_strategy) = local_general.selection_strategy {
                                        current_strategy = local_strategy;
                                        debug!("Using Group specific Config overrides");
                                    }
                                }
                            }
                        }
                    }
                }
                Err(err) => error!("Special Collection Error: {}", err),
            }
        }

        // COLLECTION: Normal Collection
        if selected_item.is_empty() {
            // Since this is the most important feature of all, propogate error and break if it fails
            let items = wallpaper::get_wallpaper_items(wallpaper_dir, extensions, Some(hour))?;
            if let Some(item) = items.get(&hour) {
                selected_item = wallpaper::select_wallpaper_item(item, extensions);

                // Local Group Config Overrides
                if let Some(local_config) = get_group_config(item) {
                    if let Some(local_general) = local_config.general {
                        if let Some(local_strategy) = local_general.selection_strategy {
                            current_strategy = local_strategy;
                            debug!("Using Group specific Config overrides");
                        }
                    }
                }
            }
        }

        // █▀ █▀▀ █░░ █▀▀ █▀▀ ▀█▀ █ █▀█ █▄░█
        // ▄█ ██▄ █▄▄ ██▄ █▄▄ ░█░ █ █▄█ █░▀█
        // Strategically Select Suitable Scene

        // Wallpaper Selection Strategies
        debug!("Exec Time: {}", format!("{:?}", start.elapsed()).cyan());
        let start = Instant::now();

        let mut interval = DEFAULT_INTERVAL_MINUTES;
        let mut refresh_strategy = WaitStrategy::Refresh;

        let item_size = selected_item.len();
        if item_size == 0 {
            warn!("No wallpaper available for {}", hour);
        } else if item_size == 1 {
            // SELECT: Fixed Time Strategy
            backend.apply_wallpaper(&selected_item[0])?;
            info!(
                "Wallpaper applied: {}",
                std::path::Path::new(&selected_item[0])
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("Unknown")
                    .bright_green()
            );
        } else if item_size > 1 {
            match current_strategy {
                GroupSelectionStrategy::Spread => {
                    let total_items = selected_item.len();
                    let max_spread_items = interval as usize;
                    if total_items > max_spread_items {
                        warn!("Too many wallpapers to spread effectively ({total_items} > {max_spread_items})");
                    }
                    let slice_duration = interval / total_items as f64;
                    let wallpaper_index = calc::get_group_index(now, total_items);

                    backend.apply_wallpaper(&selected_item[wallpaper_index as usize])?;
                    info!(
                        "Wallpaper (group) applied [{}/{}]: {}",
                        (wallpaper_index + 1).to_string().cyan(),
                        (total_items).to_string().cyan(),
                        std::path::Path::new(&selected_item[wallpaper_index])
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or("Unknown")
                            .bright_green()
                    );
                    // Overrides
                    interval = slice_duration;
                    refresh_strategy = WaitStrategy::Sleep;
                }
                GroupSelectionStrategy::Random => {
                    info!("Multiple wallpapers available for {}", hour);
                    let wallpaper_index = rand::random_range(0..item_size);
                    backend.apply_wallpaper(&selected_item[wallpaper_index])?;
                    info!(
                        "Wallpaper (group) applied [{}]: {}",
                        (wallpaper_index + 1).to_string().cyan(),
                        std::path::Path::new(&selected_item[wallpaper_index])
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or("Unknown")
                            .bright_green()
                    );
                }
            }
        }

        debug!(
            "Exec Time ({}): {}",
            backend_name,
            format!("{:?}", start.elapsed()).blue()
        );

        // █░█░█ ▄▀█ █ ▀█▀ █ █▄░█ █▀▀
        // ▀▄▀▄▀ █▀█ █ ░█░ █ █░▀█ █▄█
        // Wistful Wake Wait Workflow

        // Wait: 24 Hour Cycle Strategy
        let wait_seconds = calc::wait_time(interval, now);
        info!(
            "Waiting for: {}",
            format!(
                "{:02}:{:02}:{:02}",
                wait_seconds / SECONDS_PER_HOUR,
                (wait_seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE,
                wait_seconds % SECONDS_PER_MINUTE,
            )
            .bright_purple()
        );

        selected_item.clear();

        // CHECK: wait time discrepancies
        let full_wait_secs = (now.minute() * 60 + now.second()) + wait_seconds as u32;
        let expected_secs = (interval * 60.0).ceil() as u32;
        let diff = full_wait_secs % expected_secs;
        if diff != 0 {
            warn!(
                "Wait time misaligned by {} seconds ({} != {})",
                diff.to_string().red(),
                full_wait_secs.to_string().red(),
                expected_secs.to_string().blue()
            );
        }

        // REFRESH LOOP
        match refresh_strategy {
            WaitStrategy::Sleep => calc::sleep(wait_seconds),
            WaitStrategy::Refresh => calc::refresh(interval, now, wait_seconds),
        }

        // EXECUTE SCRIPT
        if exec_cmd.is_some() {
            let result = cmd::execute(exec_cmd.as_ref().unwrap());
            match result {
                Ok(_) => {}
                Err(err) => error!("Error executing command: {}", err),
            }
        }
    }
    Ok(())
}
