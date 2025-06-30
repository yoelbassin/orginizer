use std::path::Path;

use crate::filters::{Filter, FilterConfig, FilterKind, FilterKindType};

pub(crate) fn filters_pipeline(path: &Path, filters: &[Box<dyn Filter>]) -> bool {
    for filter in filters {
        let _ = filter.apply(path) || return false;
    }
    true
}

pub(crate) fn filter_factory(
    filter: FilterKindType,
    path: &Path,
    config: &dyn FilterConfig,
) -> Box<dyn Filter> {
    Box::new(FilterKind::from_path(filter, path, config))
}

pub(crate) fn filters_factory(
    filters: &[(FilterKindType, Box<dyn FilterConfig>)],
    path: &Path,
) -> Vec<Box<dyn Filter>> {
    filters
        .iter()
        .map(|(filter, config)| filter_factory(*filter, path, config.as_ref()))
        .collect()
}
