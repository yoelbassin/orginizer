use std::path::{Path, PathBuf};

use crate::actions::Action;

pub struct CopyAction {
    pub destination: PathBuf,
}

impl Action for CopyAction {
    fn apply(&self, path: &Path) {
        let dest = self.destination.join(path);
        println!("Copying {} to {}", path.display(), dest.display());
        println!("parent: {}", dest.parent().unwrap().display());
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::copy(path, &dest).unwrap();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
