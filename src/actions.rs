use std::any::Any;
use std::path::Path;

use crate::actions::{copy::CopyAction, delete::DeleteAction, verbose::VerboseAction};

pub trait Action: Any + 'static {
    fn apply(&self, path: &Path);
}

#[macro_use]
mod _macros;

define_actions!(
    Copy => CopyAction,
    Delete => DeleteAction,
    Verbose => VerboseAction,
);

pub mod copy;
pub mod delete;
pub mod verbose;
