use std::{fs, path::PathBuf};

use anyhow::{Result, anyhow};

const CONFIG_DIR_NAME: &str = env!("CARGO_PKG_NAME");

pub fn get_config_dir() -> Result<PathBuf> {
    let config_root = dirs::config_dir().ok_or_else(|| anyhow!("config dir not found"))?;

    let config_dir = config_root.join(CONFIG_DIR_NAME);

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    Ok(config_dir)
}
