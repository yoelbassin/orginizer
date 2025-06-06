use std::path::Path;

use crate::filters::Filter;

pub struct FileNameFilter {
    pub name: String,
}

impl Filter for FileNameFilter {
    fn apply(&self, path: &Path) -> bool {
        path.file_name().unwrap().to_str().unwrap() == self.name
    }
}
