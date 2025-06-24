use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use either::Either;
use walkdir::{DirEntry, WalkDir};

use crate::{
    filters::{Filter, FilterConfig, FilterKindType},
    organizer::filters::{filters_factory, filters_pipeline},
};

fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
}

fn exists(entry: &DirEntry) -> bool {
    entry.path().exists()
}

fn make_walker<P: AsRef<std::path::Path>>(
    path: P,
    recursive: bool,
) -> impl Iterator<Item = walkdir::Result<DirEntry>> {
    let walker = if recursive {
        WalkDir::new(path)
    } else {
        WalkDir::new(path).max_depth(1)
    };
    walker.sort_by(|a, b| a.path().cmp(b.path())).into_iter()
}

pub fn finder(
    path: &Path,
    recursive: bool,
    filters: Vec<Arc<dyn Filter>>,
) -> impl Iterator<Item = PathBuf> {
    let walker = make_walker(path, recursive);
    walker
        .filter_map(Result::ok)
        .filter(|entry| is_file(entry) && exists(entry))
        .filter(move |entry| filters_pipeline(entry.path(), &filters))
        .map(|entry| entry.path().to_path_buf())
}

fn empty_iter() -> impl Iterator<Item = PathBuf> {
    std::iter::empty()
}

pub fn duplicate_finder(
    path: &Path,
    reference: PathBuf,
    recursive: bool,
    filter_configs: &[(FilterKindType, Box<dyn FilterConfig>)],
) -> impl Iterator<Item = PathBuf> {
    if !reference.exists() {
        return Either::Left(empty_iter());
    }

    let filters: Vec<Arc<dyn Filter>> = filters_factory(filter_configs, &reference)
        .into_iter()
        .map(Arc::from)
        .collect();

    Either::Right(finder(path, recursive, filters))
}

pub fn duplicates_finder(
    path: &Path,
    reference: &PathBuf,
    recursive: bool,
    filter_configs: &[(FilterKindType, Box<dyn FilterConfig>)],
) -> impl Iterator<Item = PathBuf> {
    let walker = make_walker(reference, recursive);
    walker
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .flat_map(move |entry| {
            let reference_path = entry.path().to_path_buf();
            duplicate_finder(path, reference_path, recursive, filter_configs)
        })
}
