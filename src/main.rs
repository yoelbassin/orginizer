use organizer::{
    actions::{Action, verbose::VerboseAction},
    filters::FilterKindType,
    organizer::find_duplicates,
};

fn main() {
    let actions: Vec<Box<dyn Action>> = vec![Box::new(VerboseAction {})];
    let wanted_filters = vec![FilterKindType::FileName, FilterKindType::ImageContent];
    find_duplicates(
        &std::path::Path::new("DSC_0017-001.JPG"),
        &std::path::Path::new("/"),
        &wanted_filters,
        &actions,
    );
}
