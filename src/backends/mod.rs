use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct BackendError {
    message: String,
}
impl BackendError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}
impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BackendError: {}", self.message)
    }
}

impl Error for BackendError {}

pub trait Backend {
    fn initialize(&self) -> Result<(), BackendError>;
    fn apply_wallpaper(&self, wallpaper_path: &str) -> Result<(), BackendError>;
}

mod swww;
pub use swww::SwwwBackend;

mod feh;
pub use feh::FehBackend;

pub fn get_backend(name: &str) -> Result<Box<dyn Backend>, BackendError> {
    match name {
        "swww" => Ok(Box::new(SwwwBackend::new())),
        "feh" => Ok(Box::new(FehBackend::new())),
        _ => Err(BackendError::new("Unknown backend")),
    }
}
