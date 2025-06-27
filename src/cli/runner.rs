use super::args::Cli;
use crate::actions::{
    Action, ActionKind, copy::CopyAction, delete::DeleteAction, verbose::VerboseAction,
};
use crate::filters::file_prefix::FilePrefixFilterConfig;
use crate::filters::file_size::FileSizeFilterConfig;
use crate::filters::{FilterConfig, FilterKindType};
use crate::organizer::actions::actions_pipeline;
use crate::organizer::finder::{count_reference_files, duplicates_finder};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::sync::Arc;

enum Mode {
    Duplicates,
    Reference,
    UniqueReference,
}

#[derive(Clone)]
struct DummyConfig;
impl FilterConfig for DummyConfig {}

fn split_string_by_equal_sign(s: &str) -> (String, Option<String>) {
    let parts: Vec<&str> = s.split('=').collect();
    if parts.len() == 2 {
        (parts[0].to_string(), Some(parts[1].to_string()))
    } else {
        (s.to_string(), None)
    }
}

fn parse_filter_kind(filter: &str) -> Option<(FilterKindType, Box<dyn FilterConfig>)> {
    let (filter_type, filter_config) = split_string_by_equal_sign(filter);

    match filter_type.to_uppercase().as_str() {
        "FILE_PREFIX" => {
            let length = filter_config.unwrap().parse::<usize>().unwrap();
            Some((
                FilterKindType::FilePrefix,
                Box::new(FilePrefixFilterConfig { length }),
            ))
        }
        "NAME" => Some((FilterKindType::FileName, Box::new(DummyConfig))),
        "SIZE" => {
            let proximity = filter_config.unwrap().parse::<u64>().unwrap();
            Some((
                FilterKindType::FileSize,
                Box::new(FileSizeFilterConfig { proximity }),
            ))
        }
        "DATE_MODIFIED" => Some((FilterKindType::DateModified, Box::new(DummyConfig))),
        "DATE_CREATED" => Some((FilterKindType::DateCreated, Box::new(DummyConfig))),
        "IMAGE_CONTENT" => Some((FilterKindType::ImageContent, Box::new(DummyConfig))),
        "SKIP_SELF" => Some((FilterKindType::SkipSelf, Box::new(DummyConfig))),
        _ => panic!("Unknown filter: {}", filter_type),
    }
}

fn parse_mode(mode: &str) -> Mode {
    match mode.to_uppercase().as_str() {
        "DUPLICATES" => Mode::Duplicates,
        "REFERENCE" => Mode::Reference,
        "UNIQUE_REFERENCE" => Mode::UniqueReference,
        _ => panic!("Unknown mode: {}", mode),
    }
}

fn parse_action_kind(action: &str, progress: Option<Arc<ProgressBar>>) -> Option<ActionKind> {
    if action.to_uppercase().starts_with("COPY=") {
        let dest = action[5..].trim();
        Some(ActionKind::Copy(CopyAction {
            destination: PathBuf::from(dest),
        }))
    } else {
        match action.to_uppercase().as_str() {
            "DELETE" => Some(ActionKind::Delete(DeleteAction {})),
            "VERBOSE" => Some(ActionKind::Verbose(VerboseAction {
                progress: progress.clone(),
            })),
            _ => panic!("Unknown action: {}", action),
        }
    }
}

fn perform_action(mode: &Mode, reference_file: &PathBuf, duplicates: &Vec<PathBuf>, actions: &Vec<Box<dyn Action>>) {
    match mode {
        Mode::Duplicates => {
            for duplicate in duplicates {
                actions_pipeline(duplicate, actions);
            }
        }
        Mode::Reference => {
            if !duplicates.is_empty() {
                actions_pipeline(reference_file, actions);
            }
        }
        Mode::UniqueReference => {
            if duplicates.is_empty() {
                actions_pipeline(reference_file, actions);
            }
        }
    }
}

pub fn run_organizer(cli: &Cli) {
    // Parse filters from CLI
    let filters: Vec<(FilterKindType, Box<dyn FilterConfig>)> = cli
        .by
        .split(',')
        .filter_map(|s| parse_filter_kind(s.trim()))
        .collect();

    let reference = PathBuf::from(&cli.reference);
    let target_paths: Vec<PathBuf> = cli.targets.iter().map(|t| PathBuf::from(t)).collect();
    let reference_file_count = count_reference_files(&reference, cli.recursive);
    let pb = Arc::new(ProgressBar::new(reference_file_count as u64));
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {percent:>3}% | {msg}",
        )
        .unwrap(),
    );

    // Parse actions from CLI, passing progress bar to VerboseAction
    let actions: Vec<ActionKind> = cli
        .action
        .split(',')
        .filter_map(|s| parse_action_kind(s.trim(), Some(pb.clone())))
        .collect();

    let boxed_actions: Vec<Box<dyn Action>> = actions
        .iter()
        .cloned()
        .map(|a| Box::new(a) as Box<dyn Action>)
        .collect();

    let mode = parse_mode(&cli.mode);

    for target_path in target_paths {
        let duplicates_iter = duplicates_finder(&target_path, &reference, cli.recursive, &filters);
        for (i, (reference_file, duplicates)) in duplicates_iter.enumerate() {
            pb.set_message(format!("Checking: {}", reference_file.display()));
            pb.set_position((i + 1) as u64);
            perform_action(&mode, &reference_file, &duplicates, &boxed_actions);
        }
    }
    pb.finish_with_message("Done");
}
