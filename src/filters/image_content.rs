use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

use crate::{
    filters::{Filter, FilterConfig, FromFile},
    utils::{cache::cache_get_or_insert_with, images::get_content_hash},
};

pub struct ImageContentFilter {
    path: PathBuf,
    content_hash: Mutex<Option<String>>,
}

fn get_cache_or_insert_image_content_hash(path: &Path) -> String {
    cache_get_or_insert_with(path.to_str().unwrap(), || get_content_hash(path)).unwrap()
}

impl ImageContentFilter {
    fn get_hash(&self) -> String {
        let mut computed: std::sync::MutexGuard<'_, Option<String>> =
            self.content_hash.lock().unwrap();
        let hash =
            computed.get_or_insert_with(|| get_cache_or_insert_image_content_hash(&self.path));
        hash.clone()
    }
}

impl Filter for ImageContentFilter {
    fn apply(&self, path: &Path) -> bool {
        get_cache_or_insert_image_content_hash(path) == self.get_hash()
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
