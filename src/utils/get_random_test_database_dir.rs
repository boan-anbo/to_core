use std::path::PathBuf;
use crate::utils::id_generator::generate_id;

// save env DATABASE_URL in .env file to static variable
pub fn get_random_test_database_dir() -> String {
    let mut cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    cargo_dir.push("resources/test/db");
    cargo_dir.into_os_string().into_string().unwrap()
}
