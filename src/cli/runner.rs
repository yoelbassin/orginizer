use super::args::Cli;
use crate::actions::{
    Action, ActionKind, copy::CopyAction, delete::DeleteAction, verbose::VerboseAction,
};
use crate::filters::{FilterConfig, FilterKindType};
use crate::organizer::actions::actions_pipeline;
use crate::organizer::finder::duplicates_finder;
use std::path::PathBuf;

#[derive(Clone)]
struct DummyConfig;
impl FilterConfig for DummyConfig {}

fn parse_filter_kind(filter: &str) -> Option<FilterKindType> {
    match filter.to_uppercase().as_str() {
        "NAME" => Some(FilterKindType::FileName),
        "SIZE" => Some(FilterKindType::FileSize),
        "DATE_MODIFIED" => Some(FilterKindType::DateModified),
        "DATE_CREATED" => Some(FilterKindType::DateCreated),
        "IMAGE_CONTENT" => Some(FilterKindType::ImageContent),
        "SKIP_SELF" => Some(FilterKindType::SkipSelf),
        _ => None,
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
            _ => None,
        }
    }
}

pub fn run_organizer(cli: &Cli) {
    // Parse filters from CLI
    let filters: Vec<FilterKindType> = cli
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
        let filter_configs: Vec<(FilterKindType, Box<dyn FilterConfig>)> = filters
            .iter()
            .map(|&fk| (fk, Box::new(DummyConfig) as Box<dyn FilterConfig>))
            .collect();
        let duplicates =
            duplicates_finder(&target_path, &reference, cli.recursive, &filter_configs);
        for duplicate in duplicates {
            actions_pipeline(&duplicate, &boxed_actions);
        }
    }
}
