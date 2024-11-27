use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

use crossbeam_channel::{unbounded, Sender};
use log::{error, info};

use crate::{utils::remove_path, Payload, TRASH_GLOBS, TRASH_PATHS};

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
            let _ = tx.send(Payload::from(path));
        }

        for glob in globs {
            process_glob(glob, &tx);
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

fn process_glob(glob: impl AsRef<str>, tx: &Sender<Payload>) {
    let glob = glob.as_ref();

    match glob::glob(glob) {
        Ok(paths) => {
            for path in paths.filter_map(|result| match result {
                Ok(path) => Some(path),
                Err(e) => {
                    error!("Failed to read pattern {glob}: {e}");
                    None
                }
            }) {
                let _ = tx.send(Payload::from(path));
            }
        }
        Err(e) => error!("Invalid glob pattern {glob}: {e}"),
    };
}
