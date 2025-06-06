use std::{path::Path, time::SystemTime};

use crate::filters::Filter;

pub struct DateCreatedFilter {
    date: SystemTime,
}

impl Filter for DateCreatedFilter {
    fn apply(&self, path: &Path) -> bool {
        path.metadata().unwrap().created().unwrap() == self.date
    }
}
