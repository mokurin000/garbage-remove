use std::path::PathBuf;

use ahash::AHashSet;
use glob_match::glob_match;
use kanal::Sender;
use notify::EventHandler;

pub struct Listener {
    pub tx: Sender<PathBuf>,
    pub paths: AHashSet<PathBuf>,
    pub globs: Vec<String>,
}

impl EventHandler for Listener {
    fn handle_event(&mut self, event: notify::Result<notify::Event>) {
        let Ok(event) = event else { return };
        for path in event.paths {
            let path_str = path.to_string_lossy();
            if self.paths.contains(&path)
                || self.globs.iter().any(|glob| glob_match(glob, &path_str))
            {
                let _ = self.tx.send(path);
            }
        }
    }
}
