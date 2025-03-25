use super::Backend;
use std::error::Error;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub struct SwwwBackend;

impl SwwwBackend {
    pub fn new() -> Self {
        Self
    }

    fn is_available() -> bool {
        Command::new("swww")
            .arg("query")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

impl Backend for SwwwBackend {
    fn initialize(&self) -> Result<(), Box<dyn Error>> {
        // Swwww can take a bit while to start
        for _ in 0..5 {
            if Self::is_available() {
                return Ok(());
            }
            sleep(Duration::from_secs(1));
        }
        Err(("swww is not installed or running").into())
    }

    fn apply_wallpaper(&self, wallpaper_path: &str) -> Result<(), Box<dyn Error>> {
        let status = Command::new("swww")
            // TODO: Make args configurable and remove hardcoded values
            .args(["img", wallpaper_path, "-t", "fade"])
            .status()
            .map_err(|_| ("Failed to execute swww"))?;
        if status.success() {
            Ok(())
        } else {
            Err(("Failed to apply wallpaper with swww").into())
        }
    }

    // Simple non-blocking implementation
    /* fn apply_wallpaper(&self, wallpaper_path: &str) -> Result<(), Box<dyn Error>> {
        Command::new("swww")
            .args(["img", wallpaper_path, "-t", "fade"])
            .spawn()?;
        Ok(())
    } */

    // NOTE: Vec<&'static str> would be a great fit for modifying list on runtime (push(), remove()).
    // using &[&str] (Static Slice of String) instead since it avoids heap allocation.
    fn supported_extensions(&self) -> &[&str] {
        &["jpg", "jpeg", "png", "gif", "webp", "bmp", "pnm", "tga", "tiff"]
    }
}
