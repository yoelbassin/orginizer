use std::path::Path;

use crate::actions::{copy::CopyAction, delete::DeleteAction, verbose::VerboseAction};

pub trait Action {
    fn apply(&self, path: &Path);
}

pub enum ActionKind {
    Copy(CopyAction),
    Delete(DeleteAction),
    Verbose(VerboseAction),
}

impl Action for ActionKind {
    fn apply(&self, path: &Path) {
        match self {
            ActionKind::Copy(action) => action.apply(path),
            ActionKind::Delete(action) => action.apply(path),
            ActionKind::Verbose(action) => action.apply(path),
        }
    }
}

pub mod copy;
pub mod delete;
pub mod verbose;
