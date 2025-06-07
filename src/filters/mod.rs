use std::path::Path;

use date_created::DateCreatedFilter;
use date_modified::DateModifiedFilter;
use file_name::FileNameFilter;
use file_size::FileSizeFilter;
use image_content::ImageContentFilter;
use skip_self::SkipSelfFilter;

pub trait Filter {
    fn apply(&self, path: &Path) -> bool;
}

pub trait FromFile {
    fn new_from_file(path: &Path) -> Self;
}

pub enum FilterKind {
    FileSize(FileSizeFilter),
    FileName(FileNameFilter),
    DateModified(DateModifiedFilter),
    DateCreated(DateCreatedFilter),
    ImageContent(ImageContentFilter),
    SkipSelf(SkipSelfFilter),
}

#[derive(Copy, Clone)]
pub enum FilterKindType {
    FileName,
    FileSize,
    DateModified,
    DateCreated,
    ImageContent,
    SkipSelf,
}

impl FilterKind {
    fn as_filter(&self) -> &dyn Filter {
        match self {
            FilterKind::FileSize(f) => f,
            FilterKind::FileName(f) => f,
            FilterKind::DateModified(f) => f,
            FilterKind::DateCreated(f) => f,
            FilterKind::ImageContent(f) => f,
            FilterKind::SkipSelf(f) => f,
        }
    }

    pub fn from_path(kind: FilterKindType, path: &std::path::Path) -> Self {
        match kind {
            FilterKindType::FileName => FilterKind::FileName(FileNameFilter::new_from_file(path)),
            FilterKindType::FileSize => FilterKind::FileSize(FileSizeFilter::new_from_file(path)),
            FilterKindType::DateModified => {
                FilterKind::DateModified(DateModifiedFilter::new_from_file(path))
            }
            FilterKindType::DateCreated => {
                FilterKind::DateCreated(DateCreatedFilter::new_from_file(path))
            }
            FilterKindType::ImageContent => {
                FilterKind::ImageContent(ImageContentFilter::new_from_file(path))
            }
            FilterKindType::SkipSelf => FilterKind::SkipSelf(SkipSelfFilter::new_from_file(path)),
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
pub mod skip_self;
