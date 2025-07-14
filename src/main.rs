use std::time::Duration;

use compio::time::sleep;
use garbage_remove::utils::clean_up_once;
use garbage_remove::watcher::fs_watching;
use garbage_remove::{config::Config, utils::read_config, Result};
use spdlog::{error, info};

#[compio::main]
async fn main() -> Result<()> {
    let Config {
        paths,
        globs,
        watch_path,
    } = read_config().inspect_err(|e| error!("Failed to read initial config: {e}"))?;
    info!("paths: {paths:?}");
    info!("globs: {globs:?}");

    match watch_path {
        Some(watch_path) => {
            if watch_path.is_relative() {
                error!("relative watch path is not allowed!");
                return Ok(());
            }
            info!("start-up clean-up...");
            clean_up_once(&paths, &globs).await?;
            info!("starting fs watcher...");
            fs_watching(watch_path, paths, globs).await
        }
        None => loop {
            clean_up_once(&paths, &globs).await?;
            sleep(Duration::from_secs(30)).await;
        },
    }
}
