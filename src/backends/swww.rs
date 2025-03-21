use super::{Backend, BackendError};
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
    fn initialize(&self) -> Result<(), BackendError> {
        // Swwww can take a bit while to start
        for _ in 0..5 {
            if Self::is_available() {
                return Ok(());
            }
            sleep(Duration::from_secs(1));
        }
        return Err(BackendError::new("swww is not installed or running"));
    }

    fn apply_wallpaper(&self, wallpaper_path: &str) -> Result<(), BackendError> {
        let status = Command::new("swww")
            .args(["img", wallpaper_path, "-t", "center"])
            .status()
            .map_err(|_| BackendError::new("Failed to execute swww"))?;

        if status.success() {
            Ok(())
        } else {
            Err(BackendError::new("Failed to apply wallpaper with swww"))
        }
    }
}
