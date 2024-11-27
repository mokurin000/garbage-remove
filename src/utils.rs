use std::{
    fs::{remove_dir_all, remove_file},
    io::ErrorKind,
};

use log::{debug, error};

use crate::Payload;

pub fn remove_path(path: &Payload) {
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
