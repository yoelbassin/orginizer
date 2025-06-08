use super::args::{Cli, build_exclude_patterns, parse_actions, parse_filters};
use super::progress::{count_files, create_progress_bar, setup_ctrlc_flag};
use indicatif::ProgressBar;
use organizer::organizer::{create_filters_from_path, make_walker};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

fn setup_and_get_progress_bar(
    reference: &std::path::PathBuf,
    recursive: bool,
    globset: &organizer::utils::misc::GlobSet,
) -> Arc<ProgressBar> {
    let file_count = count_files(reference, recursive, globset);
    Arc::new(create_progress_bar(file_count))
}

fn check_interrupted(running: &Arc<AtomicBool>, pb: Option<&ProgressBar>) {
    if !running.load(std::sync::atomic::Ordering::SeqCst) {
        if let Some(pb) = pb {
            pb.finish_and_clear();
        }
        eprintln!("Interrupted by user");
        std::process::exit(130);
    }
}

fn process_target(
    reference: &std::path::Path,
    target: &std::path::Path,
    filters: &Vec<organizer::filters::FilterKindType>,
    actions: &Vec<Box<dyn organizer::actions::Action>>,
    globset: &organizer::utils::misc::GlobSet,
    recursive: Option<bool>,
    pb: &Arc<ProgressBar>,
    running: &Arc<AtomicBool>,
    reverse: bool,
) {
    let walker = make_walker(reference, recursive, globset)
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file());
    for entry in walker {
        check_interrupted(running, Some(pb));
        pb.set_message(entry.path().display().to_string());
        if !entry.path().exists() {
            pb.inc(1);
            continue;
        }
        let filters_from_source = create_filters_from_path(entry.path(), filters);
        if reverse {
            organizer::organizer::handle_reverse_duplicates(
                &entry,
                target,
                &filters_from_source,
                actions,
                globset,
                recursive,
            );
        } else {
            organizer::organizer::handle_normal_duplicates(
                target,
                &filters_from_source,
                actions,
                globset,
                recursive,
            );
        }
        pb.inc(1);
    }
}

pub fn run_organizer(cli: &Cli) {
    let running = setup_ctrlc_flag();
    let filters = parse_filters(&cli.by, cli.skip_self);
    let exclude_patterns = build_exclude_patterns(&cli.exclude, &cli.targets);
    let globset = organizer::utils::misc::make_globset(exclude_patterns.clone());
    let recursive = Some(cli.recursive);
    let pb = setup_and_get_progress_bar(&cli.reference, cli.recursive, &globset);
    let actions = parse_actions(&cli.action, cli.verbose, cli.dry_run, Some(pb.clone()));
    for target in &cli.targets {
        process_target(
            &cli.reference,
            target,
            &filters,
            &actions,
            &globset,
            recursive,
            &pb,
            &running,
            cli.reverse,
        );
        check_interrupted(&running, Some(&pb));
    }
    pb.finish_with_message("done");
}
