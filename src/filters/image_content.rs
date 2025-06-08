use std::path::{Path, PathBuf};

use crate::{
    filters::{Filter, FromFile},
    utils::images::get_content_hash,
};

pub struct ImageContentFilter {
    path: PathBuf,
    content_hash: Option<String>,
}

impl Filter for ImageContentFilter {
    // Use rawloader package to load the image content
    fn apply(&self, path: &Path) -> bool {
        let hash = if let Some(ref hash) = self.content_hash {
            hash.clone()
        } else {
            get_content_hash(&self.path)
        };
        hash == get_content_hash(path)
    }
}

impl ImageContentFilter {
    pub fn lazy_load(path: &Path) -> Self {
        let content_hash = get_content_hash(path);
        Self { path: path.to_path_buf(), content_hash: Some(content_hash) }
    }
}

impl FromFile for ImageContentFilter {
    fn new_from_file(path: &Path) -> Self {
        Self { path: path.to_path_buf(), content_hash: None }
    }
}
