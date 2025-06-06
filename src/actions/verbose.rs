use std::path::Path;

use crate::actions::Action;

pub struct VerboseAction {}

impl Action for VerboseAction {
    fn apply(&self, path: &Path) {
        println!("{}", path.display());
    }
}
