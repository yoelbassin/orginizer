use std::path::Path;

use crate::filters::{Filter, FilterConfig, FromFile};

pub struct FilePrefixFilterConfig {
    pub length: usize,
}

impl FilterConfig for FilePrefixFilterConfig {}

pub struct FilePrefixFilter {
    pub prefix: String,
}

impl Filter for FilePrefixFilter {
    fn apply(&self, path: &Path) -> bool {
        path.file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with(&self.prefix)
    }
}

impl FromFile for FilePrefixFilter {
    fn new_from_file(path: &Path, config: &dyn FilterConfig) -> Self {
        let config = config.downcast_ref::<FilePrefixFilterConfig>().unwrap();
        let prefix = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .chars()
            .take(config.length)
            .collect();
        Self { prefix }
    }
}
