use std::path::Path;

pub fn file_extension_lowercase(path: &Path) -> Option<String> {
    path.extension()
        .map(|ext| ext.to_string_lossy().to_string().to_lowercase())
}
