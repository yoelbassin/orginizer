use std::path::Path;

use crate::filters::{Filter, FromFile};

pub struct FileNameFilter {
    pub name: String,
}

impl Filter for FileNameFilter {
    fn apply(&self, path: &Path) -> bool {
        path.file_name().unwrap().to_str().unwrap() == self.name
    }
}

impl FromFile for FileNameFilter {
    fn new_from_file(path: &Path) -> Self {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        Self { name }
    }
}
