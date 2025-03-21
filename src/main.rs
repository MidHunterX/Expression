mod backends;
use backends::get_backend;
use rand::random_range;
use std::fs;

fn main() {
    let backend_name = "swww";

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

    let wallpaper_dir = "/home/midhunter/Mid_Hunter/customization/wallpaper/";

    let entries = match fs::read_dir(wallpaper_dir) {
        Ok(e) => e,
        Err(e) => {
            println!("Error reading directory: {}", e);
            std::process::exit(1);
        }
    };

    let wallpapers: Vec<String> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .map(|path| path.display().to_string())
        .collect();

    if wallpapers.is_empty() {
        println!("Error: No wallpapers found in {}", wallpaper_dir);
        std::process::exit(1);
    }

    let selected_wallpaper: &str = &wallpapers[random_range(..wallpapers.len())];

    backend
        .apply_wallpaper(selected_wallpaper)
        .expect("Failed to apply wallpaper");
}
