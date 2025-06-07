use std::any::Any;
use std::path::Path;

use crate::actions::{copy::CopyAction, delete::DeleteAction, verbose::VerboseAction};

pub trait Action: Any + 'static {
    fn apply(&self, path: &Path);
    fn as_any(&self) -> &dyn Any;
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
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub mod copy;
pub mod delete;
pub mod verbose;
