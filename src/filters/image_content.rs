use std::path::Path;

use crate::{filters::Filter, utils::images::get_content_hash};

pub struct ImageContentFilter {
    pub content_hash: String,
}

impl Filter for ImageContentFilter {
    // Use rawloader package to load the image content
    fn apply(&self, path: &Path) -> bool {
        let content_hash = get_content_hash(path);
        content_hash == self.content_hash
    }
}
