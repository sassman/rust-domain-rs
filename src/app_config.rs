use anyhow::{Context, Result};
use app_dirs2::{app_root, AppDataType, AppInfo};
use clap::{crate_authors, crate_name};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

const APP_INFO: AppInfo = AppInfo {
    name: crate_name!(),
    author: crate_authors!(),
};

pub const CONFIG_FILE_NAME: &str = "app-config.toml";

#[derive(Deserialize, Serialize, Default)]
pub struct AppConfig {
    pub provider: String,
    pub token: String,
}

impl AppConfig {
    pub fn from_path(path: &Path) -> Result<AppConfig> {
        if path.exists() {
            let cfg = fs::read_to_string(path)?;
            Ok(if cfg.trim().is_empty() {
                AppConfig::default()
            } else {
                toml::from_str(&cfg)?
            })
        } else {
            Ok(Default::default())
        }
    }

    pub fn load() -> Result<AppConfig> {
        Self::from_path(app_config_path()?.as_path())
    }
}

/// todo(migrate) to https://docs.rs/app_dirs2/2.3.2/app_dirs2/
pub(crate) fn app_config_path() -> Result<PathBuf> {
    let root = app_root(AppDataType::UserConfig, &APP_INFO)
        .context("Error when accessing / creating the app config root folder")?;
    Ok(root.join(CONFIG_FILE_NAME))
}

pub fn save(config: &AppConfig) -> Result<()> {
    let path = app_config_path()?;
    let content = toml::to_string(config)?;
    fs::write(path, content).map_err(Into::<anyhow::Error>::into)
}
