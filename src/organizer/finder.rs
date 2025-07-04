use std::path::{Path, PathBuf};

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

pub fn finder<Filters: AsRef<[Box<dyn Filter>]>>(
    path: &Path,
    recursive: bool,
    filters: Filters,
) -> impl Iterator<Item = PathBuf> {
    let walker = make_walker(path, recursive);
    walker
        .filter_map(Result::ok)
        .filter(|entry| is_file(entry) && exists(entry))
        .filter(move |entry| filters_pipeline(entry.path(), filters.as_ref()))
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

    let filters = filters_factory(filter_configs, &reference);

    Either::Right(finder(path, recursive, filters))
}

pub fn duplicates_finder(
    path: &Path,
    reference: &PathBuf,
    recursive: bool,
    filter_configs: &[(FilterKindType, Box<dyn FilterConfig>)],
) -> impl Iterator<Item = (PathBuf, Vec<PathBuf>)> {
    let walker = make_walker(reference, recursive);
    walker
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(move |entry| {
            let reference_path = entry.path().to_path_buf();
            let duplicates: Vec<PathBuf> =
                duplicate_finder(path, reference_path.clone(), recursive, filter_configs).collect();
            (reference_path, duplicates)
        })
}
