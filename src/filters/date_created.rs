use std::{path::Path, time::SystemTime};

use crate::filters::{Filter, FromFile};

pub struct DateCreatedFilter {
    date: SystemTime,
}

impl Filter for DateCreatedFilter {
    fn apply(&self, path: &Path) -> bool {
        path.metadata().unwrap().created().unwrap() == self.date
    }
}

impl FromFile for DateCreatedFilter {
    fn new_from_file(path: &Path) -> Self {
        let date = path.metadata().unwrap().created().unwrap();
        Self { date }
    }
}