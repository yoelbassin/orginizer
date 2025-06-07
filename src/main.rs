use std::path::Path;

use organizer::organizer::find_all_duplicates_in_folder;
use organizer::utils::misc::make_globset;
use organizer::{
    actions::{Action, verbose::VerboseAction},
    filters::FilterKindType,
};

fn main() {
    let actions: Vec<Box<dyn Action>> = vec![Box::new(VerboseAction {})];
    let wanted_filters = vec![FilterKindType::FileName, FilterKindType::ImageContent, FilterKindType::SkipSelf];
    let globset = make_globset(["**/src/**"]);
    find_all_duplicates_in_folder(
        &Path::new("fuck"),
        &Path::new("/"),
        &wanted_filters,
        &actions,
        &globset,
        None,
    );
}
