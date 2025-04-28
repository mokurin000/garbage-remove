use std::{
    fs::{remove_dir_all, remove_file},
    io::ErrorKind,
    path::Path,
};

use log::{debug, error, info};

use crate::{config::Config, Result};

pub fn remove_path(path: impl AsRef<Path>) {
    let path = path.as_ref();
    debug!("Received path: {}", path.to_string_lossy());

    if path.is_relative() {
        error!("relative path is not allowed: {}", path.to_string_lossy());
        return;
    }

    match if path.is_dir() {
        remove_dir_all(path)
    } else {
        remove_file(path)
    } {
        Ok(_) => {
            info!("Removed {}", path.to_string_lossy())
        }
        Err(e) => {
            if e.kind() != ErrorKind::NotFound {
                error!("Failed to remove {}: {e}", path.to_string_lossy());
            }
        }
    }
}

pub fn read_config() -> Result<Config> {
    let config_raw = std::fs::read_to_string("config.toml").unwrap_or(String::new());
    let config = toml::from_str(&config_raw)?;
    std::fs::write("config.toml", toml::to_string_pretty(&config)?)?;
    Ok(config)
}
