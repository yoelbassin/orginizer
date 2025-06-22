use std::path::Path;

use crate::filters::{Filter, FilterConfig, FromFile};

pub struct FileNameFilter {
    pub name: String,
}

impl Filter for FileNameFilter {
    fn apply(&self, path: &Path) -> bool {
        path.file_name().unwrap().to_str().unwrap().to_lowercase() == self.name.to_lowercase()
    }
}

impl FromFile for FileNameFilter {
    fn new_from_file(path: &Path, _: &dyn FilterConfig) -> Self {
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .to_lowercase();
        Self { name }
    }
}
