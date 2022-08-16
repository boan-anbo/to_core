use std::io::Take;
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Pool, Row, Sqlite};
use sqlx::sqlite::{SqlitePool, SqliteRow};
use uuid::Uuid;
use crate::to::textual_object::TextualObject;
use crate::utils::generate_id;

/// create empty database
async fn create_empty_database(db_path: &str) {
    sqlx::sqlite::Sqlite::create_database(db_path).await.unwrap();
}

/// create table on the empty database
async fn create_initial_table(pool: &Pool<Sqlite>) {
    /// if not exists, create table `textual_objects`, with UUID as primary key, sid as text, source_name as text, and update date and modification date, and json field for JSONB
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS textual_objects (
    id             UUID PRIMARY KEY,
    source_id      TEXT,
    source_name    TEXT,
    source_id_type TEXT,
    source_path    TEXT,
    store_info     TEXT,
    store_path     TEXT,
    created        TIMESTAMP,
    updated        TIMESTAMP,
    json           JSONB
        )"
    )
        .execute(pool)
        .await
        .unwrap();
}

/// connect to the database
async fn connect_to_database(db_path: &str) -> Pool<Sqlite> {
    let pool = SqlitePool::connect(db_path).await;
    match pool {
        Ok(pool) => pool,
        Err(e) => {
            /// quiet throw
            panic!("{:?}", e);
        }
    }
}

/// check if database exists
async fn database_exists(db_path: &str) -> Result<bool, sqlx::Error> {
    let options = Sqlite::database_exists(db_path).await;
    options
}

/// drop database
async fn drop_database(db_path: &str) -> Result<(), sqlx::Error> {
    let options = Sqlite::drop_database(db_path).await;
    options
}

/// remove all tables from database
async fn remove_all_tables(pool: &Pool<Sqlite>) {
    sqlx::query("DROP TABLE IF EXISTS textual_objects")
        .execute(pool)
        .await
        .unwrap();
}

/// reset database without deleting it
async fn reset_database(db_path: &str) {
    let pool = connect_to_database(db_path).await;
    remove_all_tables(&pool).await;
    create_initial_table(&pool).await;
}

/// seed 10 textual objects into database
async fn seed_random_data(pool: &Pool<Sqlite>) {
    let mut sid = String::new();
    for _ in 0..10 {
        sid.clear();
        let mut rng = rand::thread_rng();
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
        insert_textual_object_with_pool(&pool, &textual_object).await;
    }
}

// store textual object into database
async fn insert_textual_object_with_pool(pool: &Pool<Sqlite>, textual_object: &TextualObject) {
    // insert textual object into database
    let insert_query = sqlx::query!(
        "INSERT INTO textual_objects (id, ticket_id, source_id, source_id_type, source_path, store_info, store_path, created, updated, json) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
        )",
        textual_object.id,
        textual_object.ticket_id,
        textual_object.source_id,
        textual_object.source_id_type,
        textual_object.source_path,
        textual_object.store_info,
        textual_object.store_url,
        textual_object.created,
        textual_object.updated,
        textual_object.json
    );
    insert_query.execute(pool).await.unwrap();

}

// read textual object from database
async fn find_textual_object_by_id(pool: &Pool<Sqlite>, id: &Uuid) -> TextualObject {
    let mut textual_object_row = sqlx::query(
        "SELECT * FROM textual_objects WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool)
    .await.unwrap();

    let textual_object = TextualObject {
        id: textual_object_row.get("id"),
        ticket_id: textual_object_row.get("ticket_id"),
        source_id: textual_object_row.get("source_id"),
        source_id_type: textual_object_row.get("source_id_type"),
        source_path: textual_object_row.get("source_path"),
        store_info: textual_object_row.get("store_info"),
        store_url: textual_object_row.get("store_url"),
        source_name: textual_object_row.get("source_name"),
        created: textual_object_row.get("created"),
        updated: textual_object_row.get("updated"),
        json: textual_object_row.get("json"),
    };
    textual_object
}




/// reset with seeded database
async fn reset_database_with_random_data(db_path: &str) {
    /// create database if not exists
    reset_database(db_path).await;
    /// connect to database
    let pool = connect_to_database(db_path).await;

    seed_random_data(&pool).await;
}

/// unit tests
#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde_json::{json, Value};
    use sqlx::Row;
    use super::*;
    use tokio_test;

    static DB_PATH: &str = "resources/test/test_to_core.db";

    /// test create_empty_database
    #[tokio::test]
    async fn test_create_empty_database() {
        create_empty_database(DB_PATH).await;
    }

    /// test connect_to_database
    #[tokio::test]
    async fn connect_to_database_test() {
        let db_path = DB_PATH;
        let pool = connect_to_database(db_path).await;
        /// handle pool Result
        assert_eq!(pool.is_closed(), false);
        pool.close().await;
        assert_eq!(pool.is_closed(), true);
    }

    /// test database_exists
    #[tokio::test]
    async fn database_exists_test() {
        let db_path = DB_PATH;
        /// create database
        create_empty_database(db_path).await;
        let exists = database_exists(db_path).await;
        /// handle exists Result
        match exists {
            Ok(exists) => {
                assert_eq!(exists, true);
            }
            Err(e) => {
                /// throw error
                panic!("database_exists is err: {:?}", e);
            }
        }
    }

    /// test drop_database
    #[tokio::test]
    async fn drop_database_test() {
        let db_path = DB_PATH;
        /// create database
        create_empty_database(db_path).await;
        /// drop database
        let drop_result = drop_database(db_path).await;
        /// handle drop_result Result
        match drop_result {
            Ok(drop_result) => {
                let exists = database_exists(db_path).await;
                /// handle exists Result
                match exists {
                    Ok(exists) => {
                        assert_eq!(exists, false);
                    }
                    Err(e) => {
                        /// throw error
                        panic!("database_exists is err: {:?}", e);
                    }
                }
            }
            Err(e) => {
                /// throw error
                panic!("drop_database is err: {:?}", e);
            }
        }
    }

    /// test remove_all_tables
    #[tokio::test]
    async fn remove_all_tables_test() {
        let db_path = DB_PATH;
        /// create database
        create_empty_database(db_path).await;
        /// connect to database
        let pool = connect_to_database(db_path).await;
        /// match handle pool
        /// remove all data
        remove_all_tables(&pool).await;
    }

    /// test create_initial_table
    #[tokio::test]
    async fn create_initial_table_test() {
        let db_path = DB_PATH;
        /// create database
        create_empty_database(db_path).await;
        /// connect to database
        let pool = connect_to_database(db_path).await;
        /// create initial table
        create_initial_table(&pool).await;
    }

    /// test reset_database
    #[tokio::test]
    async fn reset_database_test() {
        let db_path = DB_PATH;
        /// create database
        create_empty_database(db_path).await;
        /// reset database
        reset_database(db_path).await;
    }

    /// test seed_random_data
    #[tokio::test]
    async fn seed_random_data_test() {
        let db_path = DB_PATH;
        /// create database
        create_empty_database(db_path).await;
        /// seed random data
        let pool = connect_to_database(db_path).await;
    }

    /// test reset with seeded database
    #[tokio::test]
    async fn reset_database_with_random_data_test() {
        let db_path = DB_PATH;
        /// reset database
        reset_database_with_random_data(db_path).await;
    }

    // test write textual object to database
    #[tokio::test]
    async fn write_textual_object_to_database_test() {
        let db_path = DB_PATH;
        /// create database
        create_empty_database(db_path).await;
        /// connect to database
        let pool = connect_to_database(db_path).await;
        /// create initial table
        create_initial_table(&pool).await;
        /// create textual object
        let textual_object = TextualObject {
            id: Uuid::new_v4(),
            ticket_id: generate_id(),
            source_id: "source_id".to_string(),
            source_id_type: "test".to_string(),
            source_path: "test".to_string(),
            store_info: "test".to_string(),
            store_url: "test".to_string(),
            source_name: "test".to_string(),
            created:  Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
            json: sqlx::types::Json(Value::Null),
        };
        /// write textual object to database
        insert_textual_object_with_pool(&pool, &textual_object).await;
    }

    // test read textual object from database
    #[tokio::test]
    async fn read_textual_object_from_database_test() {
        let db_path = DB_PATH;
        /// create database
        create_empty_database(db_path).await;
        /// connect to database
        let pool = connect_to_database(db_path).await;
        /// create initial table
        create_initial_table(&pool).await;
        /// create textual object
        let uuid = Uuid::new_v4();
        let textual_object = TextualObject {
            id: uuid,
            ticket_id: generate_id(),
            source_id: "source_id".to_string(),
            source_id_type: "test".to_string(),
            source_path: "test".to_string(),
            store_info: "test".to_string(),
            store_url: "test".to_string(),
            source_name: "test".to_string(),
            created:  Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
            json: sqlx::types::Json(Value::Null),
        };
        print!("{:?}", &uuid);
        /// write textual object to database
        insert_textual_object_with_pool(&pool, &textual_object).await;
        /// read textual object from database
        let textual_object_read = find_textual_object_by_id(&pool, &textual_object.id).await;
        /// handle textual_object_read Result
        assert_eq!(textual_object_read.id, uuid);
    }
}


