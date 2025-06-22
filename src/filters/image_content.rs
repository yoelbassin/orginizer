use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

use crate::{
    filters::{Filter, FilterConfig, FromFile},
    utils::images::get_content_hash,
};

pub struct ImageContentFilter {
    path: PathBuf,
    content_hash: Mutex<Option<String>>,
}

impl ImageContentFilter {
    fn get_hash(&self) -> String {
        let mut cached = self.content_hash.lock().unwrap();
        let hash = cached.get_or_insert_with(|| get_content_hash(&self.path));
        hash.clone()
    }
}

impl Filter for ImageContentFilter {
    fn apply(&self, path: &Path) -> bool {
        get_content_hash(path) == self.get_hash()
    }
}

impl FromFile for ImageContentFilter {
    fn new_from_file(path: &Path, _: &dyn FilterConfig) -> Self {
        Self {
            path: path.to_path_buf(),
            content_hash: Mutex::new(None),
        }
    }
}
