use std::path::Path;

use crate::actions::Action;

pub struct DeleteAction {}

impl Action for DeleteAction {
    fn apply(&self, path: &Path) {
        std::fs::remove_file(path).unwrap();
    }
}
