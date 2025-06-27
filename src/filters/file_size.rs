use std::path::Path;

use crate::filters::{Filter, FilterConfig, FromFile};

pub struct FileSizeFilterConfig {
    pub proximity: u64,
}

impl FilterConfig for FileSizeFilterConfig {}

pub struct FileSizeFilter {
    size: u64,
    proximity: u64,
}

impl Filter for FileSizeFilter {
    fn apply(&self, path: &Path) -> bool {
        let size = path.metadata().unwrap().len();
        size >= self.size - self.proximity && size <= self.size + self.proximity
    }
}

impl FromFile for FileSizeFilter {
    fn new_from_file(path: &Path, config: &dyn FilterConfig) -> Self {
        let config = config.downcast_ref::<FileSizeFilterConfig>().unwrap();
        let size = path.metadata().unwrap().len();
        Self {
            size,
            proximity: config.proximity,
        }
    }
}
