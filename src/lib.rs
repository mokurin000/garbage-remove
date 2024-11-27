use std::{
    borrow::Cow,
    error::Error,
    path::{Path, PathBuf},
    sync::OnceLock,
};

pub mod config;
pub mod service;
pub mod utils;

pub type Result<T> = std::result::Result<T, Box<dyn Send + Sync + Error>>;
pub type Payload = Cow<'static, Path>;

pub static TRASH_PATHS: OnceLock<Vec<PathBuf>> = OnceLock::new();
pub static TRASH_GLOBS: OnceLock<Vec<String>> = OnceLock::new();
