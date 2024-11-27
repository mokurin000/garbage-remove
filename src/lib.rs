use std::{error::Error, path::PathBuf, sync::OnceLock};

pub mod config;
pub mod service;
pub mod utils;

pub type Result<T> = std::result::Result<T, Box<dyn Send + Sync + Error>>;

pub static TRASH_PATHS: OnceLock<Vec<PathBuf>> = OnceLock::new();
pub static TRASH_GLOBS: OnceLock<Vec<String>> = OnceLock::new();
