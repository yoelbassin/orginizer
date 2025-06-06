use organizer::{filters::file_name::FileNameFilter, organizer::search_files};

fn main() {
    let filters: Vec<Box<dyn organizer::filters::Filter>> = vec![
        Box::new(FileNameFilter {
            name: "DSC_0017-001.JPG".to_string(),
        }),
    ];
    let files = search_files(&std::path::Path::new("/"), &filters);
    println!("{:?}", files);
}
