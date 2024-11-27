use std::{
    fs::{self},
    sync::atomic::Ordering,
    thread::available_parallelism,
};

use garbage_remove::{
    config::Config, service::spawn_service, Result, ALLOW_RELATIVE, TRASH_GLOBS, TRASH_PATHS,
};
use log::info;

fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let config_raw = fs::read_to_string("config.toml").unwrap_or(String::new());
    let config = toml::from_str(&config_raw)?;
    fs::write("config.toml", toml::to_string_pretty(&config)?)?;
    let Config {
        paths,
        globs,
        interval,
        num_of_workers,
        allow_relative_path,
    } = config;
    let num_of_workers = if let Some(num) = num_of_workers {
        num.into()
    } else {
        available_parallelism().map(usize::from).unwrap_or(1)
    };

    info!("Num of workers: {num_of_workers:?}");
    info!("Interval: {}", humantime::format_duration(interval));
    info!("Paths: {paths:?}");
    info!("Globs: {globs:?}");
    info!("Allow relative path: {allow_relative_path}");

    let _ = TRASH_PATHS.set(paths);
    let _ = TRASH_GLOBS.set(globs);
    let _ = ALLOW_RELATIVE.store(allow_relative_path, Ordering::Release);

    let handles = spawn_service(num_of_workers, interval);
    for handle in handles {
        handle.join().expect("failed to join thread");
    }
    Ok(())
}
