// For value-based filters (find command)

use std::sync::Arc;

use crate::cli::parsers::split_string_by_equal_sign;

pub fn parse_filters(filters: &str) -> Vec<Arc<dyn crate::filters::Filter>> {
    filters
        .split(',')
        .map(|s| parse_value_filter_kind(s.trim()))
        .collect()
}

fn parse_value_filter_kind(filter: &str) -> Arc<dyn crate::filters::Filter> {
    let (filter_type, filter_value) = split_string_by_equal_sign(filter);
    match filter_type.to_uppercase().as_str() {
        "NAME" => Arc::new(crate::filters::file_name::FileNameFilter {
            name: filter_value.unwrap(),
        }),
        "SIZE" => Arc::new(crate::filters::file_size::FileSizeFilter {
            size: filter_value.unwrap().parse::<u64>().unwrap(),
            proximity: 0,
        }),
        "TYPE" => Arc::new(crate::filters::file_type::FileTypeFilter {
            file_type: filter_value.unwrap(),
        }),
        "PREFIX" => Arc::new(crate::filters::file_prefix::FilePrefixFilter {
            prefix: filter_value.unwrap(),
        }),
        // Add more as needed
        _ => panic!("Unknown filter: {}", filter_type),
    }
}
