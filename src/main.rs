mod backends;
use backends::get_backend;
use rand::random_range;
use std::fs;

fn main() {
    let backend_name = "swww";
    let backend = get_backend(backend_name).expect("Failed to get backend");

    backend.initialize().expect("Failed to initialize backend");

    let wallpaper_dir = "/home/midhunter/Mid_Hunter/customization/wallpaper/";
    let mut wallpapers: Vec<String> = Vec::new();

    match fs::read_dir(wallpaper_dir) {
        Ok(entries) => {
            let entries: Vec<_> = entries.map(|f| f.unwrap().path()).collect();
            for entry in entries {
                if entry.is_file() {
                    let path = entry.display().to_string();
                    wallpapers.push(path);
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    let selected_wallpaper = &wallpapers[random_range(..wallpapers.len())];

    backend
        .apply_wallpaper(selected_wallpaper)
        .expect("Failed to apply wallpaper");
}
