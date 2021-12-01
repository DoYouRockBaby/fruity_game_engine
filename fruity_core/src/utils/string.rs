use std::path::Path;

pub fn get_file_type_from_path(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);
    Some(path.extension()?.to_str()?.to_string())
}
