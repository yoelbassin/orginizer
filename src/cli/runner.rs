use super::args::{Cli, build_exclude_patterns, parse_actions, parse_filters};
use super::progress::{count_files, create_progress_bar, setup_ctrlc_flag};
use indicatif::ProgressBar;
use organizer::organizer::{create_filters_from_path, find, make_walker};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub fn run_organizer(cli: &Cli) {
    let running = setup_ctrlc_flag();
    let filters = parse_filters(&cli.by, cli.skip_self);
    let exclude_patterns = build_exclude_patterns(&cli.exclude, &cli.targets);
    let globset = organizer::utils::misc::make_globset(exclude_patterns.clone());
    let recursive = Some(cli.recursive);
    let file_count = count_files(&cli.reference, cli.recursive, &globset);
    let pb = Arc::new(create_progress_bar(file_count));
    let actions = parse_actions(&cli.action, cli.verbose, cli.dry_run, Some(pb.clone()));
    for target in &cli.targets {
        find_all_duplicates_in_folder_with_interrupt(
            &cli.reference,
            target,
            &filters,
            &actions,
            &globset,
            recursive,
            Some(pb.clone()),
            running.clone(),
        );
        if !running.load(std::sync::atomic::Ordering::SeqCst) {
            pb.finish_and_clear();
            eprintln!("Interrupted by user");
            std::process::exit(130);
        }
    }
    pb.finish_with_message("done");
}

pub fn find_all_duplicates_in_folder_with_interrupt(
    source_folder: &std::path::Path,
    destination: &std::path::Path,
    filters: &Vec<organizer::filters::FilterKindType>,
    actions: &Vec<Box<dyn organizer::actions::Action>>,
    exclude: &organizer::utils::misc::GlobSet,
    recursive: Option<bool>,
    progress: Option<Arc<ProgressBar>>,
    running: Arc<AtomicBool>,
) {
    let walker = make_walker(source_folder, recursive, exclude)
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file());
    for entry in walker {
        if !running.load(std::sync::atomic::Ordering::SeqCst) {
            if let Some(pb) = &progress {
                pb.finish_and_clear();
            }
            eprintln!("Interrupted by user");
            std::process::exit(130);
        }
        let filters_from_source = create_filters_from_path(entry.path(), filters);
        find(
            destination,
            &filters_from_source,
            actions,
            exclude,
            recursive,
        );
        if let Some(pb) = &progress {
            pb.set_message(entry.path().display().to_string());
            pb.inc(1);
        }
    }
}
