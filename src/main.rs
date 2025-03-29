use chrono::{Local, Timelike};
use expression::backends::get_backend;
use expression::config::Config;
use expression::utils::wallpaper;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let config = Config::load()?;

    let backend = get_backend(&config.general.backend)?;
    backend.initialize()?;
    let extensions = backend.supported_extensions();

    let wallpaper_dir = config.directories.wallpaper.as_str();
    // NOTE: Don't worry, JFK doesn't get Executed as defaults come from config
    let special_dir = config.directories.special.as_deref().unwrap_or(&"JFK");

    let hour = Local::now().hour() as u8;
    println!("[DEBUG] This Hour: {}", hour);
    let next_hour = (hour + 1) % 24;
    println!("[DEBUG] Next Hour: {}", next_hour);

    let mut selected_wallpaper = String::new();

    // TODO: Collection: Theme Override Strategy
    let collections = wallpaper::get_collections(wallpaper_dir)?;
    for collection in collections {
        println!("collection: {}", collection.display());
    }

    // TODO: Randomized Scope Strategy

    // TODO: Collection: Special Strategy
    let special_entries = wallpaper::get_wallpapers(special_dir, extensions);
    match special_entries {
        Ok(entries) => {
            for entry in entries {
                println!("special: {}", entry.display());
            }
        }
        Err(err) => {
            println!("[WARN] Special Collection Error: {}", err);
        }
    }

    // Collection: Fixed Time Strategy
    let entries_map = wallpaper::get_wallpaper_entries(wallpaper_dir, extensions, Some(hour))?;
    if let Some(entry_vector) = entries_map.get(&hour) {
        for entry in entry_vector {
            match entry {
                // SUB-COLLECTION
                wallpaper::WallpaperEntry::Directory(path) => {
                    // TODO: toggle sub-collection config
                    // Selection: Random Strategy
                    let sub_collection_dir = path.display().to_string();
                    let sub_entries = wallpaper::get_wallpapers(&sub_collection_dir, extensions)?;
                    let wallpaper_index = rand::random_range(0..sub_entries.len());
                    selected_wallpaper = sub_entries[wallpaper_index].display().to_string();
                    println!(
                        "[INFO] Selected Wallpaper: [{}/{}] {}",
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
                    println!(
                        "[INFO] Selected wallpaper: {}",
                        selected_wallpaper.split('/').last().unwrap()
                    );
                    break;
                }
            }
        }
    }

    // TODO: Wallpaper Selection Strategies
    // Spaced Out Time Selection Strategy
    // Random Selection Strategy

    let duration = start.elapsed();
    println!("[DEBUG] Exec Time: {:?}", duration);

    backend.apply_wallpaper(&selected_wallpaper)?;
    println!("[INFO] Wallpaper Applied Successfully");

    let duration = start.elapsed();
    println!("[DEBUG] Exec Time ({}): {:?}", backend.name(), duration);

    Ok(())
}
