use std::borrow::BorrowMut;
use std::fs;
use std::path::PathBuf;

use rand::{distributions::Alphanumeric, Rng};
use sqlx::{Pool, Row, Sqlite};
use sqlx::migrate::MigrateDatabase;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqlitePool;

use crate::db::to_db_op::insert_to;
use crate::to::to_struct::TextualObject;

pub(crate) fn join_db_path(store_directory: &str, store_file_name: &str) -> String {
    let mut path = PathBuf::new();
    path.push(store_directory);
    path.push(store_file_name);
    // check if store_file_name has '.db' extension, if not, add it
    if !store_file_name.ends_with(".db") {
        path.set_extension("db");
    }
    path.into_os_string().into_string().unwrap()
}

// main entry point to initialize database, return the path of the initialized database
pub(crate) async fn initialize_database(db_root_path: &str, db_file_name: &str) -> Result<String, sqlx::Error> {
// check if it exists and has the right table structure, if not, create it
    let db_path = join_db_path(db_root_path, db_file_name);

    // check if directory exists, if not, create it
    if !PathBuf::from(db_root_path).exists() {
        fs::create_dir_all(db_root_path).unwrap();
    }

// check if db file exists, if not, create it
    let if_exists = check_if_database_exists(&db_path).await.unwrap();
    if !if_exists {
        create_empty_database_with_path_and_filename(db_root_path, db_file_name).await;
    }
    // get pool to database
    let pool = connect_to_database(&db_path).await;
    // check if the tables are there, if not, create them
    let if_tables_exist = check_if_tables_exist(&pool).await.unwrap();
    if !if_tables_exist {
        create_initial_table(&pool).await;
    }
    pool.close().await;
    Ok(db_path)
}

async fn check_if_tables_exist(p0: &Pool<Sqlite>) -> Result<bool, sqlx::Error> {
    let stmt = sqlx::query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='textual_objects';"
    );
    let rows = stmt.fetch_all(p0).await?;
    if rows.len() > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}


// create empty database
pub(crate) async fn create_empty_database(db_path: &str) {
    Sqlite::create_database(db_path).await.unwrap();
}

// create empty database with path and filename
pub(crate) async fn create_empty_database_with_path_and_filename(root_path: &str, filename: &str) {
    let db_path = join_db_path(root_path, filename);
    Sqlite::create_database(&db_path).await.unwrap();
}

// create table on the empty database
async fn create_initial_table(pool: &Pool<Sqlite>) {
    // if not exists, create table `textual_objects`, with UUID as primary key, sid as text, source_name as text, and update date and modification date, and json field for JSONB
    sqlx::query!(
     " CREATE TABLE IF NOT EXISTS textual_objects
(
    id PRIMARY KEY                    NOT NULL,
    ticket_id      TEXT               NOT NULL,
    ticket_minimal TEXT  DEFAULT ''   NOT NULL,

    source_id      TEXT               NOT NULL,
    source_name    TEXT  DEFAULT ''   NOT NULL,
    source_id_type TEXT  DEFAULT ''   NOT NULL,
    source_path    TEXT  DEFAULT ''   NOT NULL,

    store_info     TEXT  DEFAULT ''   NOT NULL,
    store_url      TEXT  DEFAULT ''   NOT NULL,

    created        TIMESTAMP          NOT NULL,
    updated        TIMESTAMP          NOT NULL,

    json           JSONB DEFAULT '{}' NOT NULL,

    card           JSONB DEFAULT NULL,
    card_map       TEXT  DEFAULT ''   NOT NULL
)"

    )
        .execute(pool)
        .await
        .unwrap();
}


// connect to the database
pub(crate) async fn connect_to_database(db_path: &str) -> Pool<Sqlite> {
    let pool = SqlitePool::connect(db_path).await;
    match pool {
        Ok(pool) => pool,
        Err(e) => {
            // quiet throw
            panic!("{:?}", e);
        }
    }
}

// check if database exists
async fn check_if_database_exists(db_path: &str) -> Result<bool, sqlx::Error> {
    let options = Sqlite::database_exists(db_path).await;
    options
}

// drop database
pub async fn drop_database(db_path: &str) -> Result<(), sqlx::Error> {
    // delete the db file at db_path in filesystem
    let re = fs::remove_file(db_path);
    match re {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into())
    }
}

// release database


// remove all tables from database
async fn remove_all_tables(pool: &Pool<Sqlite>) {
    sqlx::query("DROP TABLE IF EXISTS textual_objects")
        .execute(pool)
        .await
        .unwrap();
}

// reset database without deleting it
pub(crate) async fn reset_database(db_path: &str) {
    let pool = connect_to_database(db_path).await;
    remove_all_tables(&pool).await;
    create_empty_database(db_path).await;
    create_initial_table(&pool).await;
    pool.close().await;
}

// seed 10 textual objects into database
async fn seed_random_data(pool: &mut PoolConnection<Sqlite>) {
    let mut sid = String::new();
    for _ in 0..10 {
        sid.clear();
        let rng = rand::thread_rng();
        let sid_new_name: Vec<u8> = rng.sample_iter(&Alphanumeric).take(10).collect();
        sid = String::from_utf8(sid_new_name).unwrap();

        let mut textual_object = TextualObject::get_sample();

        textual_object.json = sqlx::types::Json(
            serde_json::json!({
                        "test_string": "test_string_value",
                        "test_number": 1,
                        "test_boolean": true,
                        "test_null": null,
                        "test_array": [1, 2, 3],
                        "test_object": {
                            "test_string": "test_string_value",
                            "test_number": 1,
                            "test_boolean": true,
                            "test_null": null,
                            "test_array": [1, 2, 3],
                        }
                    }));
        insert_to(pool, &textual_object).await;
    }
}


// reset with seeded database
async fn reset_database_with_random_data(db_path: &str) {
    // create database if not exists
    reset_database(db_path).await;
    // connect to database
    let pool = connect_to_database(db_path).await;
    let mut connetion = pool.acquire().await.unwrap();

    seed_random_data(connetion.borrow_mut()).await;
}

// unit tests
#[cfg(test)]
mod tests {
    use crate::utils::id_generator::generate_id;

    use super::*;

    static DB_PATH_WITH_FILE_NAME: &str = "resources/test/test_to_core.db";
    static TEST_DB_PATH_WITHOUT_FILE_NAME: &str = "resources/test/db/";

    // test create_empty_database
    #[tokio::test]
    async fn test_create_empty_database() {
        create_empty_database(DB_PATH_WITH_FILE_NAME).await;
    }

    // test create_database_with_random_path
    #[tokio::test]
    async fn test_create_database_with_random_path() {
        let random_file_name = generate_id();

        let full_path = join_db_path(TEST_DB_PATH_WITHOUT_FILE_NAME, random_file_name.as_str());
        create_empty_database_with_path_and_filename(TEST_DB_PATH_WITHOUT_FILE_NAME, random_file_name.as_str()).await;
        // check if the database exists
        let options = check_if_database_exists(full_path.as_str()).await;
        assert!(options.unwrap());
        // remove the database
        drop_database(full_path.as_str()).await;
    }

    // test connect_to_database
    #[tokio::test]
    async fn connect_to_database_test() {
        let db_path = DB_PATH_WITH_FILE_NAME;
        let pool = connect_to_database(db_path).await;
        // handle pool Result
        assert_eq!(pool.is_closed(), false);
        pool.close().await;
        assert_eq!(pool.is_closed(), true);
    }

    // test database_exists
    #[tokio::test]
    async fn database_exists_test() {
        let db_path = DB_PATH_WITH_FILE_NAME;
        // create database
        create_empty_database(db_path).await;
        let exists = check_if_database_exists(db_path).await;
        // handle exists Result
        match exists {
            Ok(exists) => {
                assert_eq!(exists, true);
            }
            Err(e) => {
                // throw error
                panic!("database_exists is err: {:?}", e);
            }
        }
        // check a non-existing database
        let non_existing_db_path = join_db_path(TEST_DB_PATH_WITHOUT_FILE_NAME, uuid::Uuid::new_v4().to_string().as_str());
        let result = check_if_database_exists(non_existing_db_path.as_str()).await;
        match result {
            Ok(exists) => {
                assert_eq!(exists, false);
            }
            Err(e) => {
                // throw error
                panic!("database_exists is err: {:?}", e);
            }
        }
    }


    // test remove_all_tables
    #[tokio::test]
    async fn remove_all_tables_test() {
        let db_path = DB_PATH_WITH_FILE_NAME;
        // create database
        create_empty_database(db_path).await;
        // connect to database
        let pool = connect_to_database(db_path).await;
        // match handle pool
        // remove all data
        remove_all_tables(&pool).await;
    }

    // test create_initial_table
    #[tokio::test]
    async fn create_initial_table_test() {
        let db_path = DB_PATH_WITH_FILE_NAME;
        // create database
        create_empty_database(db_path).await;
        // connect to database
        let pool = connect_to_database(db_path).await;
        // create initial table
        create_initial_table(&pool).await;
    }

    // test reset_database
    #[tokio::test]
    async fn reset_database_test() {
        let db_path = DB_PATH_WITH_FILE_NAME;
        // reset database
        reset_database(db_path).await;
    }

    // test seed_random_data
    #[tokio::test]
    async fn seed_random_data_test() {
        let db_path = DB_PATH_WITH_FILE_NAME;
        // create database
        create_empty_database(db_path).await;
        // seed random data
        let _pool = connect_to_database(db_path).await;
    }


    // test initializing database
    #[tokio::test]
    async fn initialize_database_test() {
        let random_file_name = generate_id();
        // initialize database
        let _intialized_database = initialize_database(TEST_DB_PATH_WITHOUT_FILE_NAME, random_file_name.as_str()).await;
    }

    #[tokio::test]
    async fn remove_all_test_databases() {
        let path = TEST_DB_PATH_WITHOUT_FILE_NAME;
        let directory = PathBuf::from(path);
        // remove all files from the directory
        fs::remove_dir_all(directory).unwrap();
    }
}



