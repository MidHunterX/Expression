mod backends;
use backends::get_backend;
use config::Config;
use expression::{config, utils::wallpaper};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;

    let backend = get_backend(&config.general.backend)?;
    backend.initialize()?;

    let extensions = backend.supported_extensions();
    let wallpapers = wallpaper::get_wallpapers(&config.directories.wallpaper, &extensions)?;

    if wallpapers.is_empty() {
        return Err(format!("No wallpapers found in {}", config.directories.wallpaper).into());
    }

    // TODO: Wallpaper Sourcing Strategies
    // Special Collection Strategy
    // Fixed Time Collection Strategy
    // Themed Collection Strategy
    // Root Strategy (Wallpaper Dir)
    // Randomized Scope Strategy

    // TODO: Wallpaper Selection Strategies
    // Fixed Time Strategy
    // Spaced Out Time Strategy

    // Random Wallpaper
    let wallpaper_index = rand::random_range(0..wallpapers.len());
    let selected_wallpaper = &wallpapers[wallpaper_index];

    // TEST: Print selected wallpaper
    println!("LOG: {}", selected_wallpaper);
    println!(
        "LOG: [{}/{}] {}",
        wallpaper_index,
        wallpapers.len(),
        // .unwrap() is safe here because script stops if dir is empty
        selected_wallpaper.split('/').last().unwrap().to_string()
    );

    backend.apply_wallpaper(selected_wallpaper)?;
    Ok(())
}
