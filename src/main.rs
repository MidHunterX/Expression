mod backends;
use backends::get_backend;
use config::Config;
use expression::{config, utils::wallpaper};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let config = Config::load()?;

    let backend = get_backend(&config.general.backend)?;
    backend.initialize()?;

    // TODO: Wallpaper Sourcing Strategies
    // Special Collection Strategy
    // Fixed Time Collection Strategy
    // Themed Collection Strategy
    // Root Strategy (Wallpaper Dir)
    // Randomized Scope Strategy

    let extensions = backend.supported_extensions();
    let wallpapers = wallpaper::get_wallpapers(config.directories.wallpaper.as_str(), extensions)?;

    if wallpapers.is_empty() {
        return Err(format!("No wallpapers found in {}", config.directories.wallpaper).into());
    }

    let entries = wallpaper::get_wallpaper_entries(&config.directories.wallpaper, extensions)?;
    if entries.is_empty() {
        return Err(format!("No wallpaper entries in {}", config.directories.wallpaper).into());
    }
    /* for entry in entries {
        match entry {
            wallpaper::WallpaperEntry::File(path) => {
                if let Some(filename) = path.file_name() {
                    println!("file: {}", filename.to_string_lossy());
                }
            }
            wallpaper::WallpaperEntry::Directory(path) => println!("dir: {}", path.display()),
        }
    } */

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
