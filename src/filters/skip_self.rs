use crate::filters::{Filter, FilterConfig, FromFile};
use std::fs;
use std::path::{Path, PathBuf};

pub struct SkipSelfFilter {
    pub reference_path: PathBuf,
}

impl Filter for SkipSelfFilter {
    fn apply(&self, path: &Path) -> bool {
        match fs::canonicalize(path) {
            Ok(abs_path) => abs_path != self.reference_path,
            Err(_) => panic!("Can't canonicalize path: {}", path.display()),
        }
    }
}

impl FromFile for SkipSelfFilter {
    fn new_from_file(path: &Path, _: &dyn FilterConfig) -> Self {
        let reference_path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        Self { reference_path }
    }
}
