use crate::actions::Action;
use indicatif::ProgressBar;
use std::path::Path;
use std::sync::Arc;

pub struct VerboseAction {
    pub progress: Option<Arc<ProgressBar>>,
}

impl Action for VerboseAction {
    fn apply(&self, path: &Path) {
        let msg = format!("Found duplicate: {}", path.display());
        if let Some(pb) = &self.progress {
            pb.println(msg);
        } else {
            println!("{}", msg);
        }
    }
}
