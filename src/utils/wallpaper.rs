use std::collections::BTreeMap;
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
        .collect();

    // Bubbling error as wallpapers are required
    if wallpapers.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No wallpapers found in: {}", wallpaper_dir),
        ));
    }

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
    File(PathBuf),
    Directory(PathBuf),
}

/// Retrieves all wallpaper entries (both files and sub-collections) from a directory.
///
/// Entries must be formatted as `HH` (e.g., `05`, `12`, `23`).
/// It collects directories named after hours and files with supported extensions.
/// Returns BTreeMap with Vec of entries (sub-collections before files) for each hour.
///
/// If `time_filter` is provided, entries older than the given hour (0-23) are excluded.
pub fn get_wallpaper_entries(
    wallpaper_dir: &str,
    supported_extensions: &[&str],
    time_filter: Option<u8>,
    // NOTE: Using BTreeMap instead of HashMap to auto sort entries by key
    // Sorting HashMap is less efficient
) -> Result<BTreeMap<u8, Vec<WallpaperEntry>>, io::Error> {
    let entries = fs::read_dir(wallpaper_dir)?;
    let mut wallpaper_map: BTreeMap<u8, Vec<WallpaperEntry>> = BTreeMap::new();

    for entry in entries.flatten() {
        // NOTE: .flatten() auto skips failed results
        let path = entry.path();
        let filename = path.file_stem().and_then(|name| name.to_str());

        if let Some(filename) = filename {
            if let Ok(hour) = filename.parse::<u8>() {
                if hour >= 24 {
                    continue;
                }
                let entry_type = if path.is_dir() {
                    WallpaperEntry::Directory(path)
                } else if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
                        if supported_extensions.contains(&ext) {
                            WallpaperEntry::File(path)
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };

                // BTreeMap: dir b4 file
                let list = wallpaper_map.entry(hour).or_insert_with(Vec::new);
                if matches!(entry_type, WallpaperEntry::Directory(_)) {
                    list.insert(0, entry_type); // Push directory to front
                } else {
                    list.push(entry_type);
                }
            }
        }
    }

    // filter
    if let Some(filter_hour) = time_filter {
        wallpaper_map.retain(|&hour, _entry_vec| hour >= filter_hour);
    }

    if wallpaper_map.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No wallpaper entries found in: {}", wallpaper_dir),
        ));
    }

    Ok(wallpaper_map)
}

/// Retrieves all special objects (both entries and groups) from a directory.
///
/// Special objects can have any name.
/// It collects directories and files with supported extensions.
/// Returns BTreeMap with Vec of entries (sub-collections before files) for each objects.
pub fn get_special_entries(
    special_dir: &str,
    supported_extensions: &[&str],
) -> Result<BTreeMap<String, Vec<WallpaperEntry>>, io::Error> {
    let entries = fs::read_dir(special_dir)?;
    let mut wallpaper_map: BTreeMap<String, Vec<WallpaperEntry>> = BTreeMap::new();

    for entry in entries.flatten() {
        let path = entry.path();
        let filename = path
            .file_stem()
            .and_then(|name| name.to_str())
            .map(String::from);

        if let Some(filename) = filename {
            let entry_type = if path.is_dir() {
                WallpaperEntry::Directory(path)
            } else if path.is_file() {
                if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
                    if supported_extensions.contains(&ext) {
                        WallpaperEntry::File(path)
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            };

            // BTreeMap: dir b4 file
            let list = wallpaper_map.entry(filename).or_insert_with(Vec::new);
            if matches!(entry_type, WallpaperEntry::Directory(_)) {
                list.insert(0, entry_type); // Push directory to front
            } else {
                list.push(entry_type);
            }
        }
    }

    if wallpaper_map.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No special entries found in: {}", special_dir),
        ));
    }

    Ok(wallpaper_map)
}

// █▀ █▀▀ █░░ █▀▀ █▀▀ ▀█▀ █ █▀█ █▄░█
// ▄█ ██▄ █▄▄ ██▄ █▄▄ ░█░ █ █▄█ █░▀█

use log::info;

/// Selects a random wallpaper from a wallpaper Group.
/// Returns a tuple of (path, index, total)
pub fn select_random_entry(path: &PathBuf, extensions: &[&str]) -> Option<(String, usize, usize)> {
    // Selection: Random Strategy
    let sub_collection_dir = path.display().to_string();
    let sub_entries = get_wallpapers(&sub_collection_dir, extensions).ok()?;
    if sub_entries.is_empty() {
        return None; // Avoid panic if empty
    }

    let wallpaper_index = rand::random_range(0..sub_entries.len());
    let selected_wallpaper = sub_entries[wallpaper_index].display().to_string();

    Some((selected_wallpaper, wallpaper_index, sub_entries.len()))
}

/// Selects a wallpaper from Wallpaper Object (entry/group)
pub fn select_wallpaper(entry_vector: &Vec<WallpaperEntry>, extensions: &[&str]) -> String {
    for entry in entry_vector {
        match entry {
            // WALLPAPER GROUP
            WallpaperEntry::Directory(path) => {
                // Random Strategy
                if let Some((entry, index, total)) = select_random_entry(path, extensions) {
                    info!(
                        "Selected Wallpaper: [{}/{}] {}",
                        index,
                        total,
                        entry.split('/').last().unwrap()
                    );
                    return entry;
                }
            }
            // WALLPAPER ENTRY
            WallpaperEntry::File(path) => {
                // Fixed Time Strategy
                let selected_wallpaper = path.display().to_string();
                info!(
                    "Selected Wallpaper: {}",
                    selected_wallpaper.split('/').last().unwrap()
                );
                return selected_wallpaper;
            }
        }
    }
    String::new()
}
