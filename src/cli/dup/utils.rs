use std::path::PathBuf;

use crate::{
    actions::Action,
    cli::parsers::{DummyConfig, split_string_by_equal_sign},
    filters::{
        FilterConfig, FilterKindType, file_prefix::FilePrefixFilterConfig,
        file_size::FileSizeFilterConfig,
    },
};

pub enum Mode {
    Duplicates,
    Reference,
    UniqueReference,
}

pub fn parse_mode(mode: &str) -> Mode {
    match mode.to_uppercase().as_str() {
        "DUPLICATES" => Mode::Duplicates,
        "REFERENCE" => Mode::Reference,
        "UNIQUE_REFERENCE" => Mode::UniqueReference,
        _ => panic!("Unknown mode: {}", mode),
    }
}

pub fn perform_action_by_mode(
    mode: &Mode,
    reference_file: &PathBuf,
    duplicates: &Vec<PathBuf>,
    actions: &Vec<Box<dyn Action>>,
) {
    match mode {
        Mode::Duplicates => {
            for duplicate in duplicates {
                crate::organizer::actions::actions_pipeline(duplicate, actions);
            }
        }
        Mode::Reference => {
            if !duplicates.is_empty() {
                crate::organizer::actions::actions_pipeline(reference_file, actions);
            }
        }
        Mode::UniqueReference => {
            if duplicates.is_empty() {
                crate::organizer::actions::actions_pipeline(reference_file, actions);
            }
        }
    }
}

pub fn parse_filter_kind(filter: &str) -> Option<(FilterKindType, Box<dyn FilterConfig>)> {
    let (filter_type, filter_config) = split_string_by_equal_sign(filter);
    match filter_type.to_uppercase().as_str() {
        "PREFIX" => {
            let length = filter_config.unwrap().parse::<usize>().unwrap();
            Some((
                FilterKindType::FilePrefix,
                Box::new(FilePrefixFilterConfig { length }),
            ))
        }
        "NAME" => Some((FilterKindType::FileName, Box::new(DummyConfig))),
        "SIZE" => {
            let proximity = 0;
            Some((
                FilterKindType::FileSize,
                Box::new(FileSizeFilterConfig { proximity }),
            ))
        }
        "TYPE" => Some((FilterKindType::FileType, Box::new(DummyConfig))),
        "DATE_MODIFIED" => Some((FilterKindType::DateModified, Box::new(DummyConfig))),
        "DATE_CREATED" => Some((FilterKindType::DateCreated, Box::new(DummyConfig))),
        "IMAGE_CONTENT" => Some((FilterKindType::ImageContent, Box::new(DummyConfig))),
        "SKIP_SELF" => Some((FilterKindType::SkipSelf, Box::new(DummyConfig))),
        "EXIF_CREATED" => Some((FilterKindType::ExifCreated, Box::new(DummyConfig))),
        _ => panic!("Unknown filter: {}", filter_type),
    }
}

pub fn parse_filters(filters: &str) -> Vec<(FilterKindType, Box<dyn FilterConfig>)> {
    filters
        .split(',')
        .filter_map(|s| parse_filter_kind(s.trim()))
        .collect()
}
