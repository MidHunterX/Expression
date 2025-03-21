use std::error::Error;

// Using traits for genericity of backend behavior
// https://doc.rust-lang.org/book/ch10-02-traits.html
pub trait Backend {
    fn initialize(&self) -> Result<(), Box<dyn Error>>;
    fn apply_wallpaper(&self, wallpaper_path: &str) -> Result<(), Box<dyn Error>>;
    fn supported_extensions(&self) -> Vec<&'static str>;
}

mod swww;
pub use swww::SwwwBackend;

mod feh;
pub use feh::FehBackend;

pub fn get_backend(name: &str) -> Result<Box<dyn Backend>, Box<dyn Error>> {
    match name {
        "swww" => Ok(Box::new(SwwwBackend::new())),
        "feh" => Ok(Box::new(FehBackend::new())),
        _ => Err("Unknown backend".into()),
    }
}
