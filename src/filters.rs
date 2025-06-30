use std::{any::Any, path::Path};

use date_created::DateCreatedFilter;
use date_modified::DateModifiedFilter;
use exif_created::ExifCreatedFilter;
use file_name::FileNameFilter;
use file_prefix::FilePrefixFilter;
use file_size::FileSizeFilter;
use file_type::FileTypeFilter;
use image_content::ImageContentFilter;
use skip_self::SkipSelfFilter;

pub trait FilterConfig: Any + Send + Sync {}

impl dyn FilterConfig {
    pub fn downcast_ref<T: FilterConfig + 'static>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref::<T>()
    }
}

pub trait Filter {
    fn apply(&self, path: &Path) -> bool;
}

pub trait FromFile {
    fn new_from_file(path: &Path, config: &dyn FilterConfig) -> Self;
}

#[macro_use]
mod _macros;

define_filters!(
    FileName => FileNameFilter,
    FileSize => FileSizeFilter,
    DateModified => DateModifiedFilter,
    DateCreated => DateCreatedFilter,
    ImageContent => ImageContentFilter,
    SkipSelf => SkipSelfFilter,
    FilePrefix => FilePrefixFilter,
    ExifCreated => ExifCreatedFilter,
    FileType => FileTypeFilter,
);

pub mod date_created;
pub mod date_modified;
pub mod exif_created;
pub mod file_name;
pub mod file_prefix;
pub mod file_size;
pub mod file_type;
pub mod image_content;
pub mod skip_self;
