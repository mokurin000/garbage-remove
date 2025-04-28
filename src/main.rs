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

    let handles = spawn_service(config);
    for handle in handles {
        handle.join().expect("failed to join thread");
    }
    Ok(())
}
