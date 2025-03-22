mod backends;
use backends::get_backend;
use config::Config;
use expression::{config, utils::wallpaper::get_wallpapers};
use rand::random_range;

fn main() {
    let config = Config::load().unwrap_or_else(|e| {
        eprintln!("Failed to load config: {}", e);
        std::process::exit(1);
    });

    let backend_name = &config.general.backend;
    let wallpaper_dir = &config.directories.wallpaper;

    let backend = match get_backend(backend_name) {
        Ok(b) => b,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match backend.initialize() {
        Ok(b) => b,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Wallpaper List
    let extensions = backend.supported_extensions();
    let wallpapers = match get_wallpapers(wallpaper_dir, &extensions) {
        Ok(w) => w,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    if wallpapers.is_empty() {
        println!("Error: No wallpapers found in {}", wallpaper_dir);
        std::process::exit(1);
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
    let wallpaper_index = random_range(0..wallpapers.len());
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

    backend
        .apply_wallpaper(selected_wallpaper)
        .expect("Failed to apply wallpaper");
}
