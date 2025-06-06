use std::path::Path;

use crate::filters::Filter;

pub struct FileSizeFilter {
    size: u64,
}

impl Filter for FileSizeFilter {
    fn apply(&self, path: &Path) -> bool {
        path.metadata().unwrap().len() == self.size
    }
}
