use std::{path::Path, time::SystemTime};

use crate::filters::Filter;

pub struct DateModifiedFilter {
    date: SystemTime,
}

impl Filter for DateModifiedFilter {
    fn apply(&self, path: &Path) -> bool {
        path.metadata().unwrap().modified().unwrap() == self.date
    }
}
