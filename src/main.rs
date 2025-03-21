mod backends;
use backends::get_backend;
use rand::random_range;
use std::fs;

fn main() {
    // TODO: Read backend from a config file
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

    // TODO: Read wallpapers from a config file
    let wallpaper_dir = "/windows/Customization/Wallpaper/";
    // let wallpaper_dir = "/home/midhunter/Mid_Hunter/customization/wallpaper/24h_vibe/";

    let entries = match fs::read_dir(wallpaper_dir) {
        Ok(e) => e,
        Err(e) => {
            println!("Error reading directory: {}", e);
            std::process::exit(1);
        }
    };

    // TEST: Filter by supported extensions
    let supported_extensions = backend.supported_extensions();
    let wallpapers: Vec<String> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| supported_extensions.contains(&ext))
                    .unwrap_or(false)
        })
        .map(|path| path.display().to_string())
        .collect();

    if wallpapers.is_empty() {
        println!("Error: No wallpapers found in {}", wallpaper_dir);
        std::process::exit(1);
    }

    let wallpaper_index = random_range(0..wallpapers.len());
    let selected_wallpaper: &str = &wallpapers[wallpaper_index];

    // TEST: Print selected wallpaper
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
