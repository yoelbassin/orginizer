use std::{path::Path, time::SystemTime};

use crate::filters::{Filter, FilterConfig, FromFile};

pub struct DateModifiedFilter {
    date: SystemTime,
}

impl Filter for DateModifiedFilter {
    fn apply(&self, path: &Path) -> bool {
        path.metadata().unwrap().modified().unwrap() == self.date
    }
}

impl FromFile for DateModifiedFilter {
    fn new_from_file(path: &Path, _: &dyn FilterConfig) -> Self {
        let date = path.metadata().unwrap().modified().unwrap();
        Self { date }
    }
}
