use crate::filters::{Filter, FromFile};
use std::fs;
use std::path::{Path, PathBuf};

pub struct SkipSelfFilter {
    pub reference_path: PathBuf,
}

impl Filter for SkipSelfFilter {
    fn apply(&self, path: &Path) -> bool {
        match fs::canonicalize(path) {
            Ok(abs_path) => abs_path != self.reference_path,
            Err(_) => true, // If we can't canonicalize, don't skip
        }
    }
}

impl FromFile for SkipSelfFilter {
    fn new_from_file(path: &Path) -> Self {
        let reference_path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        Self { reference_path }
    }
}
