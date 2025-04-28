use std::error::Error;

pub mod config;
pub mod utils;
pub mod watcher;

pub type Result<T> = std::result::Result<T, Box<dyn Send + Sync + Error>>;
