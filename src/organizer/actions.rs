use std::path::Path;

use crate::actions::Action;

pub fn actions_pipeline(path: &Path, actions: &Vec<Box<dyn Action>>) {
    actions.iter().for_each(|action| action.apply(path));
}
