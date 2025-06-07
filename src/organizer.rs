use crate::{actions::verbose::VerboseAction, utils::misc::path_matches_any_glob};
use globset::GlobSet;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use crate::{
    actions::Action,
    filters::{Filter, FilterKind, FilterKindType},
};

pub fn process_filter(dir_entry: &DirEntry, filters: &Vec<Box<dyn Filter>>) -> bool {
    filters.iter().all(|filter| filter.apply(&dir_entry.path()))
}

pub fn process_actions(entry: &DirEntry, actions: &Vec<Box<dyn Action>>) {
    actions
        .iter()
        .for_each(|action| action.apply(&entry.path()));
}

pub fn make_walker<P: AsRef<std::path::Path>>(
    path: P,
    recursive: Option<bool>,
    exclude: &GlobSet,
) -> impl Iterator<Item = walkdir::Result<DirEntry>> {
    let do_recursive = recursive.unwrap_or(true);
    let walker = if do_recursive {
        WalkDir::new(path)
    } else {
        WalkDir::new(path).max_depth(1)
    };
    walker
        .into_iter()
        .filter_entry(|entry| !path_matches_any_glob(entry.path(), exclude))
}

pub fn create_filters_from_path(
    path: &Path,
    filters: &Vec<FilterKindType>,
) -> Vec<Box<dyn Filter>> {
    filters
        .iter()
        .map(|filter| Box::new(FilterKind::from_path(*filter, path)) as Box<dyn Filter>)
        .collect()
}

pub fn find(
    path: &Path,
    filters: &Vec<Box<dyn Filter>>,
    actions: &Vec<Box<dyn Action>>,
    exclude: &GlobSet,
    recursive: Option<bool>,
) {
    let walker = make_walker(path, recursive, exclude);
    walker
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| process_filter(entry, filters))
        .for_each(|entry| process_actions(&entry, actions));
}

pub fn find_duplicates(
    source: &Path,
    destination: &Path,
    filters: &Vec<FilterKindType>,
    actions: &Vec<Box<dyn Action>>,
    exclude: &GlobSet,
    recursive: Option<bool>,
) {
    let filters_from_source = create_filters_from_path(source, filters);
    find(
        destination,
        &filters_from_source,
        actions,
        exclude,
        recursive,
    );
}

fn run_verbose_if_in_actions(actions: &Vec<Box<dyn Action>>, path: &Path) {
    for action in actions {
        if let Some(verbose_action) = action.as_ref().as_any().downcast_ref::<VerboseAction>() {
            verbose_action.apply(path);
        }
    }
}

pub fn handle_reverse_duplicates(
    entry: &DirEntry,
    destination: &Path,
    filters_from_source: &Vec<Box<dyn Filter>>,
    actions: &Vec<Box<dyn Action>>,
    exclude: &GlobSet,
    recursive: Option<bool>,
) {
    let mut found = false;
    let walker_dest = make_walker(destination, recursive, exclude)
        .filter_map(Result::ok)
        .filter(|dest_entry| dest_entry.file_type().is_file())
        .filter(|dest_entry| process_filter(dest_entry, filters_from_source));
    for duplicate in walker_dest {
        run_verbose_if_in_actions(actions, duplicate.path());
        found = true;
        break;
    }
    if found {
        process_actions(entry, actions);
    }
}

pub fn handle_normal_duplicates(
    destination: &Path,
    filters_from_source: &Vec<Box<dyn Filter>>,
    actions: &Vec<Box<dyn Action>>,
    exclude: &GlobSet,
    recursive: Option<bool>,
) {
    find(
        destination,
        filters_from_source,
        actions,
        exclude,
        recursive,
    );
}

pub fn find_all_duplicates_in_folder(
    source_folder: &Path,
    destination: &Path,
    filters: &Vec<FilterKindType>,
    actions: &Vec<Box<dyn Action>>,
    exclude: &GlobSet,
    recursive: Option<bool>,
    reverse: bool,
) {
    let walker = make_walker(source_folder, recursive, exclude)
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file());
    for entry in walker {
        let filters_from_source = create_filters_from_path(entry.path(), filters);
        if reverse {
            handle_reverse_duplicates(
                &entry,
                destination,
                &filters_from_source,
                actions,
                exclude,
                recursive,
            );
        } else {
            handle_normal_duplicates(
                destination,
                &filters_from_source,
                actions,
                exclude,
                recursive,
            );
        }
    }
}
