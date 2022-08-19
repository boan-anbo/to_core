use std::path::PathBuf;

// separate full path into dir and filename
pub(crate) fn split_store_path(store_path: &str) -> (String, String) {
    let path = PathBuf::from(store_path);
    let dir = path.parent().unwrap().to_str().unwrap().to_string();
    let filename = path.file_name().unwrap().to_str().unwrap().to_string();
    (dir, filename)
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_store_path() {
        let (dir, filename) = split_store_path("/home/user/test.db");
        assert_eq!(dir, "/home/user");
        assert_eq!(filename, "test.db");
    }

    // test windows
    #[test]
    fn test_split_store_path_windows() {
        let (dir, filename) = split_store_path("C:\\home\\user\\test.db");
        assert_eq!(dir, "C:\\home\\user");
        assert_eq!(filename, "test.db");
    }

    // test many levels
    #[test]
    fn test_split_store_path_many_levels() {
        let (dir, filename) = split_store_path("/home/user/test\\test.db");
        assert_eq!(dir, "/home/user/test");
        assert_eq!(filename, "test.db");
    }
}