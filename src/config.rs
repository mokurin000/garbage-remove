use std::{num::NonZero, path::PathBuf, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub paths: Vec<PathBuf>,
    pub globs: Vec<String>,
    pub interval: Duration,
    pub num_of_workers: Option<NonZero<usize>>,
    pub allow_relative_path: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paths: vec![],
            globs: vec![],
            interval: Duration::from_secs(30),
            num_of_workers: None,
            allow_relative_path: false,
        }
    }
}
