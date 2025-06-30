use std::path::Path;

use crate::filters::{Filter, FilterConfig, FromFile};

pub struct FileTypeFilter {
    file_type: String,
}

impl Filter for FileTypeFilter {
    fn apply(&self, path: &Path) -> bool {
        path.extension().unwrap().to_str().unwrap() == self.file_type
    }
}

impl FromFile for FileTypeFilter {
    fn new_from_file(path: &Path, _: &dyn FilterConfig) -> Self {
        let file_type = path.extension().unwrap().to_str().unwrap().to_string();
        Self { file_type }
    }
}
