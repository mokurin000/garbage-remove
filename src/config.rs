use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub paths: Vec<PathBuf>,
    pub globs: Vec<String>,
    pub watch_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paths: vec![],
            globs: vec![],
            watch_path: PathBuf::from("/sdcard"),
        }
    }
}
