use std::path::Path;

use tracing::{error, info};

use crate::{config::Config, Result};

pub fn read_config() -> Result<Config> {
    let config_raw = std::fs::read_to_string("config.toml").unwrap_or(String::new());
    let config = toml::from_str(&config_raw)?;
    std::fs::write("config.toml", toml::to_string_pretty(&config)?)?;
    Ok(config)
}

pub async fn remove_path(path: impl AsRef<Path>) {
    let path = path.as_ref();

    let remove = if path.is_dir() {
        tokio::fs::remove_dir_all(&path).await
    } else {
        tokio::fs::remove_file(&path).await
    };

    let path = path.to_string_lossy();
    match remove {
        Ok(_) => {
            info!("removed: {path}");
        }
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => {
            error!("failed to remove {path}: {e}",)
        }
        _ => {}
    }
}
