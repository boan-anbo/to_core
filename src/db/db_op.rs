
use rand::{distributions::Alphanumeric, Rng};

use sqlx::migrate::MigrateDatabase;
use sqlx::{Pool, Row, Sqlite};
use sqlx::sqlite::{SqlitePool};
use uuid::Uuid;
use crate::db::to_db_op::insert_to;
use crate::to::textual_object::TextualObject;
use crate::utils::id_generator::generate_id;

// create empty database
pub(crate)  async fn create_empty_database(db_path: &str) {
    Sqlite::create_database(db_path).await.unwrap();
}

// create table on the empty database
async fn create_initial_table(pool: &Pool<Sqlite>) {
    // if not exists, create table `textual_objects`, with UUID as primary key, sid as text, source_name as text, and update date and modification date, and json field for JSONB
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS textual_objects (
    id              PRIMARY KEY NOT NULL,
    ticket_id       TEXT NOT NULL,

    source_id      TEXT NOT NULL,
    source_name    TEXT DEFAULT '' NOT NULL,
    source_id_type TEXT DEFAULT '' NOT NULL,
    source_path    TEXT DEFAULT '' NOT NULL,

    store_info     TEXT DEFAULT '' NOT NULL,
    store_url      TEXT    DEFAULT '' NOT NULL,

    created        TIMESTAMP NOT NULL ,
    updated        TIMESTAMP NOT NULL ,

    json           JSONB DEFAULT '{}' NOT NULL
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
async fn database_exists(db_path: &str) -> Result<bool, sqlx::Error> {
    let options = Sqlite::database_exists(db_path).await;
    options
}

// drop database
async fn drop_database(db_path: &str) -> Result<(), sqlx::Error> {
    let options = Sqlite::drop_database(db_path).await;
    options
}

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
}

// seed 10 textual objects into database
async fn seed_random_data(pool: &Pool<Sqlite>) {
    let mut sid = String::new();
    for _ in 0..10 {
        sid.clear();
        let rng = rand::thread_rng();
        let sid_new_name: Vec<u8> = rng.sample_iter(&Alphanumeric).take(10).collect();
        sid = String::from_utf8(sid_new_name).unwrap();

        let textual_object = TextualObject {
            id: Uuid::new_v4(),
            ticket_id: generate_id(),
            source_id: sid.clone(),
            source_id_type: "Zotero Citekey".to_string(),
            source_path: "/path/to/file.txt".to_string(),
            store_info: "store info".to_string(),
            store_url: "store url".to_string(),
            source_name: "test".to_string(),
            created: chrono::NaiveDateTime::from_timestamp(0, 0),
            updated: chrono::NaiveDateTime::from_timestamp(0, 0),

            json: sqlx::types::Json(
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
                    })),
        };
        insert_to(&pool, &textual_object).await;
    }
}




// reset with seeded database
async fn reset_database_with_random_data(db_path: &str) {
    // create database if not exists
    reset_database(db_path).await;
    // connect to database
    let pool = connect_to_database(db_path).await;

    seed_random_data(&pool).await;
}

// unit tests
#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde_json::{Value};
    
    use super::*;
    

    static DB_PATH: &str = "resources/test/test_to_core.db";

    // test create_empty_database
    #[tokio::test]
    async fn test_create_empty_database() {
        create_empty_database(DB_PATH).await;
    }

    // test connect_to_database
    #[tokio::test]
    async fn connect_to_database_test() {
        let db_path = DB_PATH;
        let pool = connect_to_database(db_path).await;
        // handle pool Result
        assert_eq!(pool.is_closed(), false);
        pool.close().await;
        assert_eq!(pool.is_closed(), true);
    }

    // test database_exists
    #[tokio::test]
    async fn database_exists_test() {
        let db_path = DB_PATH;
        // create database
        create_empty_database(db_path).await;
        let exists = database_exists(db_path).await;
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
    }


    // test remove_all_tables
    #[tokio::test]
    async fn remove_all_tables_test() {
        let db_path = DB_PATH;
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
        let db_path = DB_PATH;
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
        let db_path = DB_PATH;
        // create database
        create_empty_database(db_path).await;
        // reset database
        reset_database(db_path).await;
    }

    // test seed_random_data
    #[tokio::test]
    async fn seed_random_data_test() {
        let db_path = DB_PATH;
        // create database
        create_empty_database(db_path).await;
        // seed random data
        let _pool = connect_to_database(db_path).await;
    }

    // test reset with seeded database
    #[tokio::test]
    async fn reset_database_with_random_data_test() {
        let db_path = DB_PATH;
        // reset database
        reset_database_with_random_data(db_path).await;
    }

}



