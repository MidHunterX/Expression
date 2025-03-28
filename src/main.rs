mod backends;
use backends::get_backend;
use chrono::{Local, Timelike};
use config::Config;
use expression::{config, utils::wallpaper};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let config = Config::load()?;
    let wallpaper_dir = config.directories.wallpaper.as_str();

    let backend = get_backend(&config.general.backend)?;
    backend.initialize()?;

    // TODO: Wallpaper Sourcing Strategies
    // Special Collection Strategy
    // Fixed Time Collection Strategy
    // Themed Collection Strategy
    // Root Strategy (Wallpaper Dir)
    // Randomized Scope Strategy

    let extensions = backend.supported_extensions();

    // let wallpapers = wallpaper::get_wallpapers(wallpaper_dir, extensions)?;
    let mut selected_wallpaper = String::new();

    let collections = wallpaper::get_collections(wallpaper_dir)?;
    for collection in collections {
        println!("collection: {}", collection.display());
    }

    // Chrono Time
    let hour = Local::now().hour() as u8;

    let entries_map = wallpaper::get_wallpaper_entries(wallpaper_dir, extensions, Some(hour))?;
    if let Some(entry_vector) = entries_map.get(&hour) {
        println!("hour: {}", hour);
        for entry in entry_vector {
            match entry {
                // sub-collection
                wallpaper::WallpaperEntry::Directory(path) => {
                    // TODO: toggle config
                    // Random Selection Strategy
                    let sub_collection_dir = path.display().to_string();
                    let sub_entries = wallpaper::get_wallpapers(&sub_collection_dir, extensions)?;
                    let wallpaper_index = rand::random_range(0..sub_entries.len());
                    selected_wallpaper = sub_entries[wallpaper_index].display().to_string();
                    // TEST: Print selected wallpaper
                    println!(
                        "[DEBUG] Selected wallpaper (rand): [{}/{}] {}",
                        wallpaper_index,
                        sub_entries.len(),
                        selected_wallpaper.split('/').last().unwrap()
                    );
                    break;
                }
                // entry
                wallpaper::WallpaperEntry::File(path) => {
                    selected_wallpaper = path.display().to_string();
                    // TEST: Print selected wallpaper
                    println!(
                        "[DEBUG] Selected wallpaper: {}",
                        selected_wallpaper.split('/').last().unwrap()
                    );
                    break;
                }
            }
        }
    }

    // TODO: Wallpaper Selection Strategies
    // Fixed Time Selection Strategy
    // Spaced Out Time Selection Strategy
    // Random Selection Strategy

    // TEST: Print execution time
    let duration = start.elapsed();
    println!("[DEBUG] Exec Time: {:?}", duration);

    backend.apply_wallpaper(&selected_wallpaper)?;

    // TEST: Print execution time with backend
    let duration = start.elapsed();
    println!("[DEBUG] Exec Time ({}): {:?}", backend.name(), duration);

    Ok(())
}
