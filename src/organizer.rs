use std::{path::{Path, PathBuf}};

use walkdir::{DirEntry, WalkDir};

use crate::filters::{Filter, FilterKind, FilterKindType};

fn process_entry(dir_entry: &DirEntry, filters: &Vec<Box<dyn Filter>>) -> bool {
    filters.iter().all(|filter| filter.apply(&dir_entry.path()))
}

pub fn search_files(path: &Path, filters: &Vec<Box<dyn Filter>>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let dir_entries = WalkDir::new(path).into_iter();
    for dir_entry in dir_entries {
        if let Ok(entry) = dir_entry {
            if entry.file_type().is_file() && process_entry(&entry, filters) {
                let entry_path = entry.path().to_path_buf();
                files.push(entry_path.clone());
                println!("{}", entry_path.display());
            }
        }
    }
    files
}

fn create_filters_from_path(path: &Path, filters: &Vec<FilterKindType>) -> Vec<Box<dyn Filter>> {
    filters
        .iter()
        .map(|filter| Box::new(FilterKind::from_path(*filter, path)) as Box<dyn Filter>)
        .collect()
}

pub fn find_duplicates(source: &Path, destination: &Path, filters: &Vec<FilterKindType>) -> Vec<PathBuf> {
    let filters_from_source = create_filters_from_path(source, filters);
    let duplicates = search_files(destination, &filters_from_source);
    duplicates
}



