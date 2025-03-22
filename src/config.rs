use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub directories: DirectoryConfig,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub backend: String,
}

#[derive(Debug, Deserialize)]
pub struct DirectoryConfig {
    pub wallpaper: String,
    pub special: Option<String>,
    pub collections: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let default_config_str = include_str!("../config.toml");  // include UTF-8 config file as a &str
        let default_config: toml::Value = toml::from_str(default_config_str)?;

        let user_config = dirs::config_dir()
            .map(|path| path.join("expression/config.toml"))
            .unwrap_or_else(|| PathBuf::from(""));

        let config_str = if user_config.exists() {
            let user_config_str = fs::read_to_string(&user_config)?;
            let user_config: toml::Value = toml::from_str(&user_config_str)?;
            let merged_config = merge_toml(default_config, user_config);
            toml::to_string(&merged_config)?
        } else {
            toml::to_string(&default_config)?
        };

        let mut config: Config = toml::from_str(&config_str)?;

        if config.directories.special.is_none() {
            let wallpaper_path = Path::new(&config.directories.wallpaper);
            config.directories.special = Some(
                wallpaper_path
                    .join("special")
                    .to_string_lossy()
                    .into_owned(),
            );
        }

        if config.directories.collections.is_none() {
            let wallpaper_path = Path::new(&config.directories.wallpaper);
            config.directories.collections = Some(wallpaper_path.to_string_lossy().into_owned());
        }

        Ok(config)
    }
}

fn merge_toml(mut base: toml::Value, overrides: toml::Value) -> toml::Value {
    if let (toml::Value::Table(base_table), toml::Value::Table(override_table)) =
        (&mut base, overrides)
    {
        for (key, override_value) in override_table {
            base_table.insert(key, override_value);
        }
    }
    base
}
