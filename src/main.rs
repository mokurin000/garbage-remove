use std::path::PathBuf;

use garbage_remove::utils::remove_path;
use garbage_remove::{config::Config, utils::read_config, watcher::Listener, Result};
use notify::Watcher;
use tokio::task::spawn_blocking;
use tracing::level_filters::LevelFilter;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let Config {
        paths,
        globs,
        watch_path,
    } = read_config().inspect_err(|e| error!("Failed to read initial config: {e}"))?;
    info!("paths: {paths:?}");
    info!("globs: {globs:?}");

    if watch_path.is_relative() {
        error!("relative watch path is not allowed!");
        return Ok(());
    }

    info!("start-up clean-up...");
    for path in &paths {
        remove_path(path).await;
    }
    for glob in globs.clone() {
        let paths: Vec<PathBuf> =
            spawn_blocking(move || glob::glob(&glob).into_iter().flatten().flatten().collect())
                .await?;
        for path in paths {
            remove_path(path).await;
        }
    }

    info!("starting fs watcher...");

    let (tx, rx) = kanal::unbounded();

    let listener = Listener {
        tx,
        paths: paths.into_iter().collect(),
        globs,
    };
    let mut watcher = notify::recommended_watcher(listener)?;
    watcher.watch(&watch_path, notify::RecursiveMode::Recursive)?;

    let rx = rx.as_async();
    while let Ok(path) = rx.recv().await {
        remove_path(path).await;
    }

    Ok(())
}
