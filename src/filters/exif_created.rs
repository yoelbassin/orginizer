use std::{fs::File, io::BufReader, path::Path};

use chrono::{DateTime, Utc};

use crate::filters::{Filter, FilterConfig, FromFile};
use exif::{Exif, Tag};

pub struct ExifCreatedFilter {
    pub date_time_original: DateTime<Utc>,
}

fn compare_to_exif_tag(value: String, tag: Tag, exif: Exif) -> bool {
    let exif_value = exif
        .get_field(tag, exif::In::PRIMARY)
        .unwrap()
        .display_value()
        .to_string();
    value == exif_value
}

fn get_exif(path: &Path) -> Exif {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let exif = exif::Reader::new()
        .read_from_container(&mut reader)
        .unwrap();
    exif
}

impl Filter for ExifCreatedFilter {
    fn apply(&self, path: &Path) -> bool {
        let exif = get_exif(path);
        compare_to_exif_tag(
            self.date_time_original.to_string(),
            exif::Tag::DateTimeOriginal,
            exif,
        )
    }
}

impl FromFile for ExifCreatedFilter {
    fn new_from_file(path: &Path, _: &dyn FilterConfig) -> Self {
        let exif = get_exif(path);
        let date_time_original = exif
            .get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY)
            .unwrap()
            .display_value()
            .to_string();
        let date_time_original = DateTime::parse_from_str(&date_time_original, "%Y:%m:%d %H:%M:%S")
            .unwrap()
            .with_timezone(&Utc);
        Self { date_time_original }
    }
}
