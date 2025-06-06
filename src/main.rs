use organizer::{filters::{image_content::ImageContentFilter, FilterKindType}, organizer::{find_duplicates, search_files}, utils::images::get_content_hash};

fn main() {
    let wanted_filters = vec![FilterKindType::FileName, FilterKindType::ImageContent];
    let files = find_duplicates(&std::path::Path::new("DSC_0017-001.JPG"), &std::path::Path::new("/"), &wanted_filters);
    println!("{:?}", files);
}
