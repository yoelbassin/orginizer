use organizer::{filters::{image_content::ImageContentFilter}, organizer::search_files, utils::images::get_content_hash};

fn main() {
    let filters: Vec<Box<dyn organizer::filters::Filter>> = vec![
        Box::new(ImageContentFilter {
            content_hash: get_content_hash(&std::path::Path::new("./DSC_0017-001.JPG")),
        }),
    ];
    let files = search_files(&std::path::Path::new("."), &filters);
    println!("{:?}", files);
}
