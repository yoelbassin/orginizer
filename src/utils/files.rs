use std::path::Path;

pub fn is_directory(path: &Path) -> bool {
    path.metadata().map(|m| m.is_dir()).unwrap_or(false)
}

pub fn file_name_from_path(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_string()
}

pub fn file_extension_lowercase(path: &Path) -> Option<String> {
    path.extension()
        .map(|ext| ext.to_string_lossy().to_string().to_lowercase())
}
