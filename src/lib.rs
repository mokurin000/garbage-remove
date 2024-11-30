use std::{
    borrow::Cow,
    error::Error,
    path::Path,
    sync::atomic::AtomicBool,
};

pub mod config;
pub mod service;
pub mod utils;

pub type Result<T> = std::result::Result<T, Box<dyn Send + Sync + Error>>;
pub type Payload = Cow<'static, Path>;

pub static ALLOW_RELATIVE: AtomicBool = AtomicBool::new(false);
