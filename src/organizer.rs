use std::path::Path;

use walkdir::{DirEntry, WalkDir};

use crate::{
    actions::Action,
    filters::{Filter, FilterKind, FilterKindType},
};

fn process_filter(dir_entry: &DirEntry, filters: &Vec<Box<dyn Filter>>) -> bool {
    filters.iter().all(|filter| filter.apply(&dir_entry.path()))
}

fn process_actions(entry: &DirEntry, actions: &Vec<Box<dyn Action>>) {
    actions
        .iter()
        .for_each(|action| action.apply(&entry.path()));
}

pub fn search_files(path: &Path, filters: &Vec<Box<dyn Filter>>, actions: &Vec<Box<dyn Action>>) {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| process_filter(entry, filters))
        .for_each(|entry| process_actions(&entry, actions));
}

fn create_filters_from_path(path: &Path, filters: &Vec<FilterKindType>) -> Vec<Box<dyn Filter>> {
    filters
        .iter()
        .map(|filter| Box::new(FilterKind::from_path(*filter, path)) as Box<dyn Filter>)
        .collect()
}

pub fn find_duplicates(
    source: &Path,
    destination: &Path,
    filters: &Vec<FilterKindType>,
    actions: &Vec<Box<dyn Action>>,
) {
    let filters_from_source = create_filters_from_path(source, filters);
    search_files(destination, &filters_from_source, actions);
}
