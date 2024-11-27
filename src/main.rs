use std::{
    fs::{self},
    thread::available_parallelism,
};

use garbage_remove::{config::Config, service::spawn_service, Result, TRASH_PATHS};
use log::info;

fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let config_raw = fs::read_to_string("config.toml").unwrap_or(String::new());
    let Config {
        paths,
        interval,
        num_of_workers,
    } = toml::from_str(&config_raw)?;

    info!("Num of workers: {num_of_workers:?}");
    info!("Interval: {}", humantime::format_duration(interval));
    info!("Paths: {paths:?}");

    let _ = TRASH_PATHS.set(paths);

    let num_of_workers = if let Some(num) = num_of_workers {
        num.into()
    } else {
        available_parallelism().map(usize::from).unwrap_or(1)
    };

    let handles = spawn_service(num_of_workers, interval);
    for handle in handles {
        handle.join().expect("failed to join thread");
    }
    Ok(())
}
