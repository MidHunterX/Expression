use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
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
        let default_config = PathBuf::from("config.toml");
        let user_config = dirs::config_dir()
            .map(|path| path.join("expression/config.toml"))
            .unwrap_or(default_config.clone());

        let mut config_str = fs::read_to_string(&default_config)?;

        if user_config.exists() {
            let user_config_str = fs::read_to_string(&user_config)?;
            let user_config: toml::Value = toml::from_str(&user_config_str)?;
            let default_config: toml::Value = toml::from_str(&config_str)?;

            let merged_config = merge_toml(default_config, user_config);
            config_str = toml::to_string(&merged_config)?;
        }

        let config: Config = toml::from_str(&config_str)?;
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
