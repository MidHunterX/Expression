use std::fs;
use std::io;

/// Get all wallpapers with supported extensions in a directory
/// - Returns a vector of file paths regardless of filename
/// - Useful for getting all wallpapers in a sub-collection
///   as sub-collections might contain unstructured filenames
pub fn get_wallpapers(
    wallpaper_dir: &str,
    supported_extensions: &[&str],
) -> Result<Vec<String>, io::Error> {
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

/// Get all subcollections (HH) in a directory
pub fn get_subcollections(wallpaper_dir: &str) -> Result<Vec<String>, io::Error> {
    let entries = fs::read_dir(wallpaper_dir)?;
    let mut directories = Vec::new();

    for entry in entries {
        let entry = entry?; // Unwrap
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.chars().all(|c| c.is_numeric()) {
                    directories.push(path.display().to_string());
                }
            }
        }
    }

    Ok(directories)
}

pub enum WallpaperEntry {
    File(String),
    Directory(String),
}

/// Get all entries (wallpapers and sub-collections in the format HH) in a collection
/// TODO: Optional time argument for filtering out old entries
/// TODO: Time based sorting
pub fn get_wallpaper_entries(
    wallpaper_dir: &str,
    supported_extensions: &[&str],
) -> Result<Vec<WallpaperEntry>, io::Error> {
    let entries = fs::read_dir(wallpaper_dir)?;
    let mut wallpaper_entries = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
                if supported_extensions.contains(&ext) {
                    wallpaper_entries.push(WallpaperEntry::File(path.display().to_string()));
                }
            }
        } else if path.is_dir() {
            wallpaper_entries.push(WallpaperEntry::Directory(path.display().to_string()));
        }
    }

    Ok(wallpaper_entries)
}
