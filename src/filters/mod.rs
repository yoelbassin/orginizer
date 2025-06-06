use std::path::Path;

use file_size::FileSizeFilter;
use file_name::FileNameFilter;
use date_modified::DateModifiedFilter;
use date_created::DateCreatedFilter;
use image_content::ImageContentFilter;

pub trait Filter {
    fn apply(&self, path: &Path) -> bool;
}

pub enum FilterKind {
    FileSize(FileSizeFilter),
    FileName(FileNameFilter),
    DateModified(DateModifiedFilter),
    DateCreated(DateCreatedFilter),
    ImageContent(ImageContentFilter),
}

impl FilterKind {
    fn as_filter(&self) -> &dyn Filter {
        match self {
            FilterKind::FileSize(f) => f,
            FilterKind::FileName(f) => f,
            FilterKind::DateModified(f) => f,
            FilterKind::DateCreated(f) => f,
            FilterKind::ImageContent(f) => f,
        }
    }
}

impl Filter for FilterKind {
    fn apply(&self, path: &Path) -> bool {
        self.as_filter().apply(path)
    }
}

pub mod date_created;
pub mod date_modified;
pub mod file_name;
pub mod file_size;
pub mod image_content;
