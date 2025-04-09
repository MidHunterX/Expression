use expression::utils::wallpaper;
use std::fs;
use std::io;
mod utils;
use utils::{cleanup_test_dir, setup_test_dir};

// █░█░█ ▄▀█ █░░ █░░ █▀█ ▄▀█ █▀█ █▀▀ █▀█
// ▀▄▀▄▀ █▀█ █▄▄ █▄▄ █▀▀ █▀█ █▀▀ ██▄ █▀▄

#[test]
fn test_get_wallpapers() -> io::Result<()> {
    let test_dir = std::env::temp_dir().join("test_get_wallpapers");
    cleanup_test_dir(&test_dir); // Ensure it's clean before test
    setup_test_dir(&test_dir, &["wall1.jpg", "wall2.png", "ignore.txt"], &[])?;

    let wallpapers = wallpaper::get_wallpapers(test_dir.to_str().unwrap(), &["jpg", "png"])?;

    assert_eq!(wallpapers.len(), 2, "Expected 2 wallpapers");
    assert!(wallpapers.iter().any(|p| p.ends_with("wall1.jpg")));
    assert!(wallpapers.iter().any(|p| p.ends_with("wall2.png")));

    cleanup_test_dir(&test_dir);
    Ok(())
}

#[test]
fn test_get_wallpapers_empty() {
    let test_dir = std::env::temp_dir().join("test_get_wallpapers_empty");
    cleanup_test_dir(&test_dir);
    fs::create_dir_all(&test_dir).unwrap();

    let result = wallpaper::get_wallpapers(test_dir.to_str().unwrap(), &["jpg", "png"]);

    assert!(
        result.is_err(),
        "Expected an error for empty wallpaper directory"
    );

    cleanup_test_dir(&test_dir);
}

// █▀▀ █▀█ █░░ █░░ █▀▀ █▀▀ ▀█▀ █ █▀█ █▄░█
// █▄▄ █▄█ █▄▄ █▄▄ ██▄ █▄▄ ░█░ █ █▄█ █░▀█

#[test]
fn test_get_collections() -> io::Result<()> {
    let test_dir = std::env::temp_dir().join("test_get_collections");
    cleanup_test_dir(&test_dir);
    setup_test_dir(
        &test_dir,
        &["file1.jpg"],
        &["A_collection", "B_collection", "12"],
    )?;

    let collections = wallpaper::get_collections(test_dir.to_str().unwrap())?;

    assert_eq!(collections.len(), 2, "Expected 2 valid collections");
    assert!(collections.iter().any(|p| p.ends_with("A_collection")));
    assert!(collections.iter().any(|p| p.ends_with("B_collection")));

    cleanup_test_dir(&test_dir);
    Ok(())
}

#[test]
fn test_get_collections_empty() {
    let test_dir = std::env::temp_dir().join("test_get_collections_empty");
    cleanup_test_dir(&test_dir);
    fs::create_dir_all(&test_dir).unwrap();

    let result = wallpaper::get_collections(test_dir.to_str().unwrap());

    assert!(
        result.unwrap().is_empty(),
        "Expected no collections in an empty directory"
    );

    cleanup_test_dir(&test_dir);
}

// █ ▀█▀ █▀▀ █▀▄▀█
// █ ░█░ ██▄ █░▀░█

#[test]
fn test_get_wallpaper_items() -> io::Result<()> {
    let test_dir = std::env::temp_dir().join("test_get_wallpaper_items");
    cleanup_test_dir(&test_dir);
    setup_test_dir(&test_dir, &["00.jpg", "12.png", "23.jpeg"], &["10", "20"])?;

    let items = wallpaper::get_wallpaper_items(test_dir.to_str().unwrap(), &["jpg", "png"], None)?;

    assert_eq!(items.len(), 4, "Expected 4 wallpaper items (files & dirs)");

    cleanup_test_dir(&test_dir);
    Ok(())
}

#[test]
fn test_get_wallpaper_items_with_filter() -> io::Result<()> {
    let test_dir = std::env::temp_dir().join("test_get_wallpaper_items_filter");
    cleanup_test_dir(&test_dir);
    setup_test_dir(&test_dir, &["00.jpg", "12.png", "23.jpg"], &[])?;

    let items =
        wallpaper::get_wallpaper_items(test_dir.to_str().unwrap(), &["jpg", "png"], Some(12))?;

    assert_eq!(
        items.len(),
        2,
        "Expected 2 wallpapers after filtering by hour"
    );

    cleanup_test_dir(&test_dir);
    Ok(())
}
