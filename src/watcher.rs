use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use ahash::AHashSet;
use glob_match::glob_match;
use kanal::Sender;
use notify::{EventHandler, Watcher as _};

use crate::{utils::remove_path, Result};

pub async fn fs_watching(
    watch_path: impl AsRef<Path>,
    paths: impl IntoIterator<Item = PathBuf>,
    globs: Vec<String>,
) -> Result<()> {
    let (tx, rx) = kanal::unbounded();
    let listener = Listener {
        tx,
        paths: paths.into_iter().collect(),
        globs,
    };
    let mut watcher = notify::recommended_watcher(listener)?;
    watcher.watch(watch_path.as_ref(), notify::RecursiveMode::Recursive)?;

    let rx = rx.as_async();
    loop {
        while let Ok(path) = rx.recv().await {
            _ = remove_path(path).await;
        }
        compio::runtime::time::sleep(Duration::from_millis(500)).await;
    }
}

pub struct Listener {
    pub tx: Sender<PathBuf>,
    pub paths: AHashSet<PathBuf>,
    pub globs: Vec<String>,
}

impl EventHandler for Listener {
    fn handle_event(&mut self, event: notify::Result<notify::Event>) {
        let Ok(event) = event else { return };
        for path in event.paths {
            let path_str = path.to_string_lossy();
            if self.paths.contains(&path)
                || self.globs.iter().any(|glob| glob_match(glob, &path_str))
            {
                let _ = self.tx.send(path);
            }
        }
    }
}
