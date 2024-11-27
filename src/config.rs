use std::{num::NonZero, path::PathBuf, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub paths: Vec<PathBuf>,
    pub interval: Duration,
    pub num_of_workers: Option<NonZero<usize>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paths: vec![],
            interval: Duration::from_secs(30),
            num_of_workers: None,
        }
    }
}
