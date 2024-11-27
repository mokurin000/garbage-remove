use std::{
    fs::{remove_dir_all, remove_file},
    io::ErrorKind,
    path::PathBuf,
    thread::{self, JoinHandle},
    time::Duration,
};

use crossbeam_channel::unbounded;
use log::{debug, error, info};

use crate::{TRASH_GLOBS, TRASH_PATHS};

pub fn spawn_service(num_of_workers: usize, interval: Duration) -> Vec<JoinHandle<()>> {
    let mut handles = Vec::with_capacity(num_of_workers + 1);
    let (tx, rx) = unbounded();
    let handle = thread::spawn(move || loop {
        let Some(paths) = TRASH_PATHS.get() else {
            continue;
        };
        let Some(globs) = TRASH_GLOBS.get() else {
            continue;
        };

        for path in paths {
            let _ = tx.send(path.clone());
        }

        for glob in globs {
            match glob::glob(glob) {
                Ok(paths) => {
                    for path in paths.filter_map(|result| match result {
                        Ok(path) => Some(path),
                        Err(e) => {
                            error!("Failed to read pattern {glob} due to {e}");
                            None
                        }
                    }) {
                        let _ = tx.send(path);
                    }
                }
                Err(e) => error!("Invalid glob pattern: {glob}, reason: {e}"),
            };
        }

        thread::sleep(interval);
    });
    info!("Started producer thread");
    handles.push(handle);

    handles.extend((0..num_of_workers).into_iter().enumerate().map(|(id, _)| {
        let rx = rx.clone();
        let handle = thread::spawn(move || {
            while let Ok(path) = rx.recv() {
                remove_path(&path);
            }
        });

        info!("Started worker {id}");

        handle
    }));

    handles
}

fn remove_path(path: &PathBuf) {
    debug!("Received path: {}", path.to_string_lossy());

    if path.is_relative() {
        error!("relative path is not allowed");
        return;
    }

    if let Err(e) = if path.is_dir() {
        remove_dir_all(path)
    } else {
        remove_file(path)
    } {
        if e.kind() != ErrorKind::NotFound {
            error!("Failed to remove {}: {e}", path.to_string_lossy());
        }
    }
}
