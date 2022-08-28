use std::path::Path;

pub fn check_if_file_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    path.exists()
}
