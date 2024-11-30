use std::{
    fs::{remove_dir_all, remove_file},
    io::ErrorKind,
    sync::atomic::Ordering,
};

use log::{debug, error, info};

use crate::{config::Config, Payload, Result, ALLOW_RELATIVE};

pub fn remove_path(path: &Payload) {
    debug!("Received path: {}", path.to_string_lossy());

    let allow_relative_path = ALLOW_RELATIVE.load(Ordering::Acquire);
    if !allow_relative_path && path.is_relative() {
        error!("relative path is not allowed");
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
