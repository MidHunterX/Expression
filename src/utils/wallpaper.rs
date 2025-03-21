use std::fs;

pub fn get_wallpapers(
    wallpaper_dir: &str,
    supported_extensions: &[&str],
) -> Result<Vec<String>, std::io::Error> {
    let entries = fs::read_dir(wallpaper_dir)?;

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

    Ok(wallpapers)
}
