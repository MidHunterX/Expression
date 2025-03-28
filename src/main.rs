mod backends;
use backends::get_backend;
use config::Config;
use expression::{config, utils::wallpaper};
use std::time::Instant;
use chrono::{Local, Timelike};

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

    let wallpapers = wallpaper::get_wallpapers(wallpaper_dir, extensions)?;

    let collections = wallpaper::get_collections(wallpaper_dir)?;
    for collection in collections {
        println!("collection: {}", collection.display());
    }

    // Chrono Time
    let hour = Local::now().hour() as u8;

    let entries = wallpaper::get_wallpaper_entries(wallpaper_dir, extensions, Some(hour))?;
    for entry in entries {
        match entry {
            wallpaper::WallpaperEntry::File(path) => {
                if let Some(filename) = path.file_name() {
                    println!("file: {}", filename.to_string_lossy());
                }
            }
            wallpaper::WallpaperEntry::Directory(path) => {
                if let Some(dirname) = path.file_name() {
                    println!("dir : {}/", dirname.to_string_lossy());
                }
            }
        }
    }

    // TODO: Wallpaper Selection Strategies
    // Fixed Time Selection Strategy
    // Spaced Out Time Selection Strategy
    // Random Selection Strategy

    // Random Selection Strategy
    let wallpaper_index = rand::random_range(0..wallpapers.len());
    let selected_wallpaper = &wallpapers[wallpaper_index].display().to_string();

    // TEST: Print selected wallpaper
    println!(
        "[DEBUG] Selected wallpaper: [{}/{}] {}",
        wallpaper_index,
        wallpapers.len(),
        // .unwrap() is safe here because script stops if dir is empty
        selected_wallpaper.split('/').last().unwrap()
    );

    // TEST: Print execution time
    let duration = start.elapsed();
    println!("[DEBUG] Exec Time: {:?}", duration);

    backend.apply_wallpaper(selected_wallpaper)?;

    // TEST: Print execution time with backend
    let duration = start.elapsed();
    println!("[DEBUG] Exec Time ({}): {:?}", backend.name(), duration);

    Ok(())
}
