use crate::actions::Action;
use std::path::Path;

#[derive(Clone)]
pub struct VerboseAction {}

impl Action for VerboseAction {
    fn apply(&self, path: &Path) {
        let msg = format!("Processing: {}", path.display());
        println!("{}", msg);
    }
}
