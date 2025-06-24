use super::args::Cli;
use crate::actions::{
    Action, ActionKind, copy::CopyAction, delete::DeleteAction, verbose::VerboseAction,
};
use crate::filters::file_prefix::FilePrefixFilterConfig;
use crate::filters::{FilterConfig, FilterKindType};
use crate::organizer::actions::actions_pipeline;
use crate::organizer::finder::duplicates_finder;
use std::path::PathBuf;

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
        "SIZE" => Some((FilterKindType::FileSize, Box::new(DummyConfig))),
        "DATE_MODIFIED" => Some((FilterKindType::DateModified, Box::new(DummyConfig))),
        "DATE_CREATED" => Some((FilterKindType::DateCreated, Box::new(DummyConfig))),
        "IMAGE_CONTENT" => Some((FilterKindType::ImageContent, Box::new(DummyConfig))),
        "SKIP_SELF" => Some((FilterKindType::SkipSelf, Box::new(DummyConfig))),
        _ => panic!("Unknown filter: {}", filter_type),
    }
}

fn parse_action_kind(action: &str) -> Option<ActionKind> {
    if action.to_uppercase().starts_with("COPY=") {
        let dest = action[5..].trim();
        Some(ActionKind::Copy(CopyAction {
            destination: PathBuf::from(dest),
        }))
    } else {
        match action.to_uppercase().as_str() {
            "DELETE" => Some(ActionKind::Delete(DeleteAction {})),
            "VERBOSE" => Some(ActionKind::Verbose(VerboseAction { progress: None })),
            _ => panic!("Unknown action: {}", action),
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

    // Parse actions from CLI
    let actions: Vec<ActionKind> = cli
        .action
        .split(',')
        .filter_map(|s| parse_action_kind(s.trim()))
        .collect();

    let reference = PathBuf::from(&cli.reference);
    let boxed_actions: Vec<Box<dyn Action>> = actions
        .iter()
        .cloned()
        .map(|a| Box::new(a) as Box<dyn Action>)
        .collect();

    for target in &cli.targets {
        let target_path = PathBuf::from(target);
        let duplicates = duplicates_finder(&target_path, &reference, cli.recursive, &filters);
        for duplicate in duplicates {
            actions_pipeline(&duplicate, &boxed_actions);
        }
    }
}
