use crate::actions::{
    Action, ActionKind, copy::CopyAction, delete::DeleteAction, verbose::VerboseAction,
};
use crate::filters::FilterConfig;
use std::path::PathBuf;

#[derive(Clone)]
pub struct DummyConfig;
impl FilterConfig for DummyConfig {}

pub fn parse_targets(targets: &Vec<String>) -> Vec<PathBuf> {
    targets
        .iter()
        .map(|t| std::path::PathBuf::from(t))
        .collect()
}

pub fn split_string_by_equal_sign(s: &str) -> (String, Option<String>) {
    let parts: Vec<&str> = s.split('=').collect();
    if parts.len() == 2 {
        (parts[0].to_string(), Some(parts[1].to_string()))
    } else {
        (s.to_string(), None)
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
            "VERBOSE" => Some(ActionKind::Verbose(VerboseAction {})),
            _ => panic!("Unknown action: {}", action),
        }
    }
}

pub fn parse_actions(actions: &str) -> Vec<Box<dyn Action>> {
    let actions: Vec<crate::actions::ActionKind> = actions
        .split(',')
        .filter_map(|s| parse_action_kind(s.trim()))
        .collect();

    let boxed_actions: Vec<Box<dyn crate::actions::Action>> = actions
        .iter()
        .cloned()
        .map(|a| Box::new(a) as Box<dyn crate::actions::Action>)
        .collect();

    boxed_actions
}
