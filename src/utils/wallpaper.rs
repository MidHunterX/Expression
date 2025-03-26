use std::fs;
use std::io;
use std::path::PathBuf;

/// Get all wallpapers with supported extensions in a directory
/// - Returns a vector of file paths regardless of filename
/// - Useful for getting all wallpapers in a sub-collection
///   as sub-collections might contain unstructured filenames
pub fn get_wallpapers(
    wallpaper_dir: &str,
    supported_extensions: &[&str],
) -> Result<Vec<PathBuf>, io::Error> {
    let entries = fs::read_dir(wallpaper_dir)?;

    let wallpapers: Vec<PathBuf> = entries
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
        // .map(|path| path.display().to_string())
        .collect();

    Ok(wallpapers)
}

/// Get all collections in a directory
/// - Collections are directories which are not sub-collections (HH)
/// - So, any directories with names other than HH are considered collections
/// - Collections e.g. `Dark_Mode/`, `Nature/`, `Light_Mode/`, etc
pub fn get_collections(wallpaper_dir: &str) -> Result<Vec<PathBuf>, io::Error> {
    let entries = fs::read_dir(wallpaper_dir)?;
    let mut directories = Vec::new();

    for entry in entries {
        let entry = entry?; // Unwrap
        let path = entry.path();

        if path.is_dir() {
            let dir_name = path.file_name().and_then(|n| n.to_str());
            if let Some(name) = dir_name {
                if name.len() > 2 {
                    directories.push(path);
                } else {
                    // Edge case: Collection name has two or less letters
                    // push only if name does not start with a number
                    let first_char = name.chars().next().unwrap();
                    if !first_char.is_numeric() {
                        directories.push(path);
                    }
                }
            }
        }
    }

    Ok(directories)
}

pub enum WallpaperEntry {
    // File(&'a Path), Adding lifetimes ('a), makes it harder to store in structs
    File(PathBuf),
    Directory(PathBuf),
}

/// Retrieves all wallpaper entries (both files and sub-collections) from a directory.
///
/// Entries must be formatted as `HH` (e.g., `05`, `12`, `23`).
/// It collects directories named after hours and files with supported extensions.
///
/// If `time_filter` is provided, entries older than the given hour (0-23) are excluded.
///
/// ### Example
/// ```
/// use wallpaper::{get_wallpaper_entries, WallpaperEntry};
///
/// let path = "/path/to/collection";
/// let extensions = &["jpg", "png"];
/// let entries = get_wallpaper_entries(path, extensions, None)?;
///
/// if entries.is_empty() {
///     return Err(format!("No wallpaper entries in {}", path)));
/// }
///
/// for entry in entries {
///     match entry {
///         WallpaperEntry::Directory(path) => {
///             println!("Directory: {}", path.display());
///         }
///         WallpaperEntry::File(path) => {
///             println!("file: {}", path.display());
///         }
///     }
/// }
/// ```
pub fn get_wallpaper_entries(
    wallpaper_dir: &str,
    supported_extensions: &[&str],
    time_filter: Option<u8>,
) -> Result<Vec<WallpaperEntry>, io::Error> {
    let entries = fs::read_dir(wallpaper_dir)?;

    let mut wallpaper_entries = Vec::new();
    let mut time_based_entries = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = path.file_stem().and_then(|name| name.to_str());

        // Valid entries in format HH
        if let Some(filename) = filename {
            if filename.len() == 2 {
                if let Ok(hour) = filename.parse::<u8>() {
                    if hour < 24 {
                        time_based_entries.push((hour, path.clone()));
                    }
                }
            }
        }
    }

    time_based_entries.sort_by_key(|&(hour, _)| hour);

    if let Some(filter_hour) = time_filter {
        time_based_entries.retain(|&(hour, _)| hour >= filter_hour);
    }

    for (_, path) in time_based_entries {
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
                if supported_extensions.contains(&ext) {
                    wallpaper_entries.push(WallpaperEntry::File(path));
                }
            }
        } else if path.is_dir() {
            wallpaper_entries.push(WallpaperEntry::Directory(path));
        }
    }

    Ok(wallpaper_entries)
}
