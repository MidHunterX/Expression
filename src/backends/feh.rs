use super::{Backend, BackendError};
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
    fn initialize(&self) -> Result<(), BackendError> {
        if !Self::is_available() {
            return Err(BackendError::new("feh is not installed"))
        }
        Ok(())
    }

    fn apply_wallpaper(&self, wallpaper_path: &str) -> Result<(), BackendError> {
        let status = Command::new("feh")
            .args(["--bg-center", wallpaper_path])
            .status()
            .map_err(|_| BackendError::new("Failed to execute feh"))?;

        if status.success() {
            Ok(())
        } else {
            Err(BackendError::new("Failed to apply wallpaper with feh"))
        }
    }
}
