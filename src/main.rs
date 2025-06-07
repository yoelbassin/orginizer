use std::path::Path;

use organizer::utils::misc::make_globset;
use organizer::{
    actions::{Action, verbose::VerboseAction},
    filters::FilterKindType,
    organizer::find_duplicates,
};

fn main() {
    let actions: Vec<Box<dyn Action>> = vec![Box::new(VerboseAction {})];
    let wanted_filters = vec![FilterKindType::FileName, FilterKindType::ImageContent];
    let globset = make_globset(["**/src/**"]);
    find_duplicates(
        &Path::new("DSC_0017-001.JPG"),
        &Path::new("/"),
        &wanted_filters,
        &actions,
        &globset,
    );
}
