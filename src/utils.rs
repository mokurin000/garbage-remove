use std::{
    fs::{remove_dir_all, remove_file},
    io::ErrorKind,
    sync::atomic::Ordering,
};

use log::{debug, error};

use crate::{Payload, ALLOW_RELATIVE};

pub fn remove_path(path: &Payload) {
    debug!("Received path: {}", path.to_string_lossy());

    let allow_relative_path = ALLOW_RELATIVE.load(Ordering::Relaxed);
    if !allow_relative_path && path.is_relative() {
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
