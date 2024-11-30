use std::thread::available_parallelism;

use garbage_remove::{service::spawn_service, utils::read_config, Result};
use log::{error, info};

fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let config = match read_config() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to read initial config: {e}");
            Err(e)?
        }
    };
    info!("Initial config: {config:?}");

    let num_of_workers = if let Some(num) = config.num_of_workers {
        num.into()
    } else {
        available_parallelism().map(usize::from).unwrap_or(1)
    };

    info!("Num of workers: {num_of_workers:?}");

    let handles = spawn_service(num_of_workers, config);
    for handle in handles {
        handle.join().expect("failed to join thread");
    }
    Ok(())
}
