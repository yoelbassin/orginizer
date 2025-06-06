use std::path::{Path, PathBuf};

use crate::actions::Action;

pub struct CopyAction {
    pub destination: PathBuf,
}

impl Action for CopyAction {
    fn apply(&self, path: &Path) {
        std::fs::copy(path, &self.destination).unwrap();
    }
}
