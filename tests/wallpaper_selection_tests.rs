use expression::utils::wallpaper;
use std::fs;
mod utils;
use utils::{cleanup_test_dir, setup_test_dir};

// █▀ █▀▀ █░░ █▀▀ █▀▀ ▀█▀ █ █▀█ █▄░█
// ▄█ ██▄ █▄▄ ██▄ █▄▄ ░█░ █ █▄█ █░▀█

#[test]
fn test_select_random_entry_valid() {
    let test_dir = std::env::temp_dir().join("test_select_random_entry_valid");
    cleanup_test_dir(&test_dir);
    setup_test_dir(&test_dir, &["img1.jpg", "img2.png"], &[]).unwrap();

    let result = wallpaper::select_random_entry(&test_dir, &["jpg", "png"]);
    assert!(result.is_some(), "Expected a selected wallpaper");

    let (path, index, total) = result.unwrap();
    assert!(path.ends_with(".jpg") || path.ends_with(".png"));
    assert!(index < total && total == 2);

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_select_random_entry_empty_dir() {
    let test_dir = std::env::temp_dir().join("test_select_random_entry_empty_dir");
    cleanup_test_dir(&test_dir);
    fs::create_dir_all(&test_dir).unwrap();

    let result = wallpaper::select_random_entry(&test_dir, &["jpg", "png"]);
    assert!(result.is_none(), "Expected None for empty dir");

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_select_wallpaper_with_group_entry() {
    let test_dir = std::env::temp_dir().join("test_select_wallpaper_with_group_entry");
    cleanup_test_dir(&test_dir);
    setup_test_dir(&test_dir, &["wallpaper1.jpg", "wallpaper2.png"], &[]).unwrap();

    let items = vec![wallpaper::WallpaperItem::Group(test_dir.clone())];
    let selected = wallpaper::select_wallpaper_item(&items, &["jpg", "png"]);

    assert!(
        selected.len() == 2,
        "Expected selected wallpaper from group"
    );

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_select_wallpaper_with_direct_entry() {
    let test_dir = std::env::temp_dir().join("test_select_wallpaper_with_direct_entry");
    cleanup_test_dir(&test_dir);
    let wallpaper_path = test_dir.join("specific_wall.jpg");
    fs::create_dir_all(&test_dir).unwrap();
    fs::write(&wallpaper_path, "test").unwrap();

    let items = vec![wallpaper::WallpaperItem::Entry(wallpaper_path.clone())];
    let selected = wallpaper::select_wallpaper_item(&items, &["jpg", "png"]);

    assert_eq!(selected[0], wallpaper_path.display().to_string());

    cleanup_test_dir(&test_dir);
}
