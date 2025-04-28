use garbage_remove::{config::Config, utils::read_config, watcher::Listener, Result};
use notify::Watcher;
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

    Ok(())
}
