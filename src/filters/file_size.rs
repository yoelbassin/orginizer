use std::path::Path;

use crate::filters::{Filter, FilterConfig, FromFile};

pub struct FileSizeFilter {
    size: u64,
}

impl Filter for FileSizeFilter {
    fn apply(&self, path: &Path) -> bool {
        path.metadata().unwrap().len() == self.size
    }
}

impl FromFile for FileSizeFilter {
    fn new_from_file(path: &Path, _: &dyn FilterConfig) -> Self {
        let size = path.metadata().unwrap().len();
        Self { size }
    }
}
