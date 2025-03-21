use super::Backend;
use std::error::Error;
use std::process::Command;

pub struct FehBackend;

impl FehBackend {
    pub fn new() -> Self {
        Self
    }

    fn is_available() -> bool {
        Command::new("feh")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

impl Backend for FehBackend {
    fn initialize(&self) -> Result<(), Box<dyn Error>> {
        if !Self::is_available() {
            return Err(("feh is not installed").into());
        }
        Ok(())
    }

    fn apply_wallpaper(&self, wallpaper_path: &str) -> Result<(), Box<dyn Error>> {
        let status = Command::new("feh")
            .args(["--bg-center", wallpaper_path])
            .status()
            .map_err(|_| ("Failed to execute feh"))?;

        if status.success() {
            Ok(())
        } else {
            Err(("Failed to apply wallpaper with feh").into())
        }
    }
}
