use std::{path::{Path, PathBuf}};

use walkdir::{DirEntry, WalkDir};

use crate::filters::Filter;

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