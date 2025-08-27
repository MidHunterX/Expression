use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use toml;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GroupSelectionStrategy {
    Spread,
    Random,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub directories: DirectoryConfig,
    pub special_entries: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub backend: String,
    pub enable_special: bool,
    pub group_selection_strategy: GroupSelectionStrategy,
    pub execute_on_change: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DirectoryConfig {
    pub wallpaper: String,

    #[serde(default)]
    pub special: String,

    #[serde(default)]
    pub collections: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let default_config_str = include_str!("../config.toml"); // include UTF-8 config file as a &str
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

        let wallpaper_path = Path::new(&config.directories.wallpaper);

        if config.directories.special.is_empty() {
            config.directories.special = wallpaper_path
                .join("special")
                .to_string_lossy()
                .into_owned();
        }

        if config.directories.collections.is_empty() {
            config.directories.collections = wallpaper_path.to_string_lossy().into_owned();
        }

        // Expand dir paths
        config.directories.wallpaper = expand_path(&config.directories.wallpaper);
        config.directories.special = expand_path(&config.directories.special);
        config.directories.collections = expand_path(&config.directories.collections);

        Ok(config)
    }
}

fn merge_toml(mut base: toml::Value, overrides: toml::Value) -> toml::Value {
    if let (toml::Value::Table(base_table), toml::Value::Table(override_table)) =
        (&mut base, overrides)
    {
        for (key, override_value) in override_table {
            match base_table.get(&key) {
                // If key exists in both, recursively merge if both are tables
                Some(base_value) => {
                    if let (toml::Value::Table(_), toml::Value::Table(_)) =
                        (base_value, &override_value)
                    {
                        // Create new merged value
                        let merged = merge_toml(base_value.clone(), override_value);
                        base_table.insert(key, merged);
                    } else {
                        // Else replace value
                        base_table.insert(key, override_value);
                    }
                }
                // If key not exist in base, insert override value
                None => {
                    base_table.insert(key, override_value);
                }
            }
        }
    }
    base
}

fn expand_path(path: &str) -> String {
    shellexpand::full(path)
        .unwrap_or_else(|_| path.into())
        .into_owned()
}

// █▀▀ █▀█ █▀█ █░█ █▀█   █▀▀ █▀█ █▄░█ █▀▀ █ █▀▀
// █▄█ █▀▄ █▄█ █▄█ █▀▀   █▄▄ █▄█ █░▀█ █▀░ █ █▄█
// Override Config for Wallpaper Groups

use crate::core::wallpaper::WallpaperItem;
use log2::error;

#[derive(Debug, Deserialize, Clone)]
pub struct GroupConfig {
    pub general: Option<GeneralGroupConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeneralGroupConfig {
    pub selection_strategy: Option<GroupSelectionStrategy>,
}

/// Selects config from the first Group in the list of WallpaperItem.
/// Returns None if there is no Group or config.toml is not present/invalid.
pub fn get_group_config(current_wallpaper_items: &Vec<WallpaperItem>) -> Option<GroupConfig> {
    let config_file = "config.toml";
    for item in current_wallpaper_items {
        if let WallpaperItem::Group(path) = item {
            let config_path = path.join(config_file);
            if config_path.exists() && config_path.is_file() {
                match fs::read_to_string(&config_path) {
                    Ok(content) => match toml::from_str::<GroupConfig>(&content) {
                        Ok(config) => return Some(config),
                        Err(err) => {
                            error!("Failed to parse {}: {}", config_path.display(), err);
                            return None;
                        }
                    },
                    Err(err) => {
                        error!("Failed to read {}: {}", config_path.display(), err);
                        return None;
                    }
                }
            }
        }
    }
    None
}
