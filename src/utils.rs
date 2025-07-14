use std::path::{Path, PathBuf};

use compio::runtime::spawn_blocking;
use spdlog::{error, info};

use crate::{config::Config, Result};

pub async fn clean_up_once(
    paths: impl IntoIterator<Item = impl AsRef<Path>>,
    globs: impl IntoIterator<Item = &String>,
) -> Result<()> {
    for path in paths {
        _ = remove_path(path).await;
    }

    for glob in globs.into_iter().cloned() {
        let paths: Vec<PathBuf> =
            spawn_blocking(move || glob::glob(&glob).into_iter().flatten().flatten().collect())
                .await
                .map_err(|_| "failed to join")?;
        for path in paths {
            _ = remove_path(path).await;
        }
    }

    Ok(())
}

pub fn read_config() -> Result<Config> {
    let config_raw = std::fs::read_to_string("config.toml").unwrap_or_default();
    let config = toml::from_str(&config_raw)?;
    std::fs::write("config.toml", toml::to_string_pretty(&config)?)?;
    Ok(config)
}

pub async fn remove_path(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    let remove = if path.is_dir() {
        let path = path.to_path_buf();
        compio::runtime::spawn_blocking(move || std::fs::remove_dir_all(&path))
            .await
            .map_err(|_| "failed to join")?
    } else {
        compio::fs::remove_file(&path).await
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

    Ok(())
}
