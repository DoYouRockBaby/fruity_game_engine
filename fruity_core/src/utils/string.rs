use std::path::Path;

/// Extract the file type from a file path
///
/// # Arguments
/// * `file_path` - The file path
///
pub fn get_file_type_from_path(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);
    Some(path.extension()?.to_str()?.to_string())
}
