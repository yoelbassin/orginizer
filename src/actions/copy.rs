use std::path::{Component, Path, PathBuf};

use crate::actions::Action;

fn join_paths(base: impl AsRef<Path>, append: impl AsRef<Path>) -> PathBuf {
    let mut buf = base.as_ref().to_path_buf();

    for comp in append.as_ref().components() {
        use Component::*;
        match comp {
            RootDir | Prefix(_) => continue,
            CurDir => {}
            ParentDir => {
                buf.pop();
            }
            Normal(segment) => buf.push(segment),
        }
    }

    buf
}

#[derive(Clone)]
pub struct CopyAction {
    pub destination: PathBuf,
}

impl Action for CopyAction {
    fn apply(&self, path: &Path) {
        let dest = join_paths(&self.destination, path);
        println!("Copying {} to {}", path.display(), dest.display());
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::copy(path, &dest).unwrap();
    }
}
