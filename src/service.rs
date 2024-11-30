use std::thread::{self, JoinHandle};

use crossbeam_channel::{unbounded, Sender};
use log::{error, info};

use crate::{
    config::Config,
    utils::{read_config, remove_path},
    Payload,
};

pub fn spawn_service(num_of_workers: usize, context: Config) -> Vec<JoinHandle<()>> {
    let Config {
        paths,
        globs,
        interval,
        ..
    } = context;
    let mut handles = Vec::with_capacity(num_of_workers + 1);
    let (tx, rx) = unbounded();
    let handle = thread::spawn(move || {
        let mut cur_interval = interval;
        let mut cur_paths = paths;
        let mut cur_globs = globs;
        loop {
            match read_config() {
                Ok(Config {
                    paths,
                    globs,
                    interval,
                    ..
                }) => {
                    if paths != cur_paths {
                        info!("New paths: {paths:?}");
                        cur_paths = paths;
                    }
                    if globs != cur_globs {
                        info!("New globs: {globs:?}");
                        cur_globs = globs;
                    }
                    if interval != cur_interval {
                        info!("New interval: {}", humantime::format_duration(interval));
                        cur_interval = interval;
                    }
                }
                Err(e) => {
                    error!("Failed to read new config: {e}");
                }
            }

            for path in cur_paths.iter() {
                let _ = tx.send(Payload::from(path.clone()));
            }

            for glob in cur_globs.iter() {
                process_glob(glob, &tx);
            }

            thread::sleep(cur_interval);
        }
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
