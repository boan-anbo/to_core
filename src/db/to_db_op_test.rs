// test groups
#[cfg(test)]
mod test {
    use std::borrow::{Borrow, BorrowMut};
    use std::{env, fs};
    use std::path::PathBuf;

    use chrono::Utc;
    use dotenv::dotenv;
    use serde_json::Value;
    use sqlx::Connection;
    use uuid::Uuid;

    use crate::db::db_op::{connect_to_database, initialize_database, reset_database};
    use crate::db::to_db_op::{find_to_by_id, find_to_by_ticket_id, insert_to};
    use crate::to::textual_object::TextualObject;
    use crate::utils::id_generator::generate_id;

    // save env DATABASE_URL in .env file to static variable
    fn get_random_database_dir() -> String {
        let mut cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cargo_dir.push("resources/test/db");
        cargo_dir.into_os_string().into_string().unwrap()
    }


    async fn get_random_database() -> String {
        let random_id = generate_id();
        let initialized_database_url = initialize_database(&get_random_database_dir(), &random_id).await;
        initialized_database_url.unwrap()
    }

    // test write textual object to database
    #[tokio::test]
    async fn write_textual_object_to_database_test() {
        // create database
        let random_database = get_random_database().await;
        // create textual object
        let textual_object = TextualObject {
            id: Uuid::new_v4(),
            ticket_id: generate_id(),
            source_id: "source_id".to_string(),
            source_id_type: "test".to_string(),
            source_path: "test".to_string(),
            store_info: "test".to_string(),
            store_url: "test".to_string(),
            source_name: "test".to_string(),
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
            json: sqlx::types::Json(Value::Null),
        };
        // write textual object to database
        // get pool
        let pool = connect_to_database(&random_database).await;
        let mut conn = pool.acquire().await.unwrap();
        let _uuid = insert_to(conn.borrow_mut(), &textual_object).await;
    }

    // test read textual object from database
    #[tokio::test]
    async fn read_textual_object_from_database_test() {
        // create database
        let random_database = get_random_database().await;

        let pool = connect_to_database(&random_database).await;
        // create textual object
        let uuid = Uuid::new_v4();
        let json = serde_json::json!({
                    "test_key": "test_value",
                    "empty_key": "",
                    "null_key": null,
                    "array_key": [1, 2, 3],
            "number_key": 1,
            "boolean_key": true,
                });
        let textual_object_insert = TextualObject {
            id: uuid,
            ticket_id: generate_id(),

            source_id: "source_id".to_string(),
            source_id_type: "source_id_type".to_string(),
            source_path: "source_path".to_string(),
            source_name: "source_name".to_string(),

            store_info: "store_info".to_string(),
            store_url: "store_url".to_string(),

            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),

            json: sqlx::types::Json(json),
        };
        print!("{:?}", &uuid);
        let mut conn = pool.acquire().await.unwrap();
        // write textual object to database
        insert_to(conn.borrow_mut(), &textual_object_insert).await;
        // read textual object from database

        let mut conn2 = pool.acquire().await.unwrap();

        let textual_object_read = find_to_by_id(conn2.borrow_mut(), &textual_object_insert.id).await.unwrap();



        // handle textual_object_read Result
        assert_eq!(textual_object_read.id, uuid);
        assert_eq!(textual_object_read.ticket_id, textual_object_insert.ticket_id);
        assert_eq!(textual_object_read.source_id, textual_object_insert.source_id);
        assert_eq!(textual_object_read.source_id_type, textual_object_insert.source_id_type);
        assert_eq!(textual_object_read.source_path, textual_object_insert.source_path);
        assert_eq!(textual_object_read.store_info, textual_object_insert.store_info);
        assert_eq!(textual_object_read.store_url, textual_object_insert.store_url);
        assert_eq!(textual_object_read.source_name, textual_object_insert.source_name);
        assert_eq!(textual_object_read.created, textual_object_insert.created);
        assert_eq!(textual_object_read.updated, textual_object_insert.updated);
        assert_eq!(textual_object_read.json, textual_object_insert.json);
        // check the integrity of the json field
        let json_read = textual_object_read.json.0.as_object().unwrap();
        let json_insert = textual_object_insert.json.0.as_object().unwrap();
        assert_eq!(json_read["test_key"], json_insert["test_key"]);
        assert_eq!(json_read["empty_key"], json_insert["empty_key"]);
        assert_eq!(json_read["null_key"], json_insert["null_key"]);
        assert_eq!(json_read["array_key"], json_insert["array_key"]);
        assert_eq!(json_read["number_key"], json_insert["number_key"]);
        assert_eq!(json_read["boolean_key"], json_insert["boolean_key"]);
    }


    // test find_by_id
    #[tokio::test]
    async fn find_textual_object_from_database_test() {
        // create database
        let random_database = get_random_database().await;

        let pool = connect_to_database(&random_database).await;

        let mut conn = pool.acquire().await.unwrap();
        // when result is non
        let textual_object_read = find_to_by_id(conn.borrow_mut(), &Uuid::new_v4()).await;
        // delete textual object from database
        assert!(textual_object_read.is_none());

        // when result is one
        let to_insert_uuid = Uuid::new_v4();
        let to_insert = TextualObject {
            id: to_insert_uuid,
            ticket_id: generate_id(),
            source_id: "source_id".to_string(),
            source_id_type: "test".to_string(),
            source_path: "test".to_string(),
            store_info: "test".to_string(),
            store_url: "test".to_string(),
            source_name: "test".to_string(),
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
            json: sqlx::types::Json(Value::Null),
        };
        let mut conn = pool.acquire().await.unwrap();
        let received_id = insert_to(conn.borrow_mut(), &to_insert).await;
        assert_eq!(received_id, to_insert_uuid);
        let mut conn2 = pool.acquire().await.unwrap();
        let found_to = find_to_by_id(conn.borrow_mut(), &to_insert_uuid).await;
        assert!(found_to.is_some());
        assert_eq!(found_to.unwrap().id, to_insert_uuid);
    }

    // test find_by_ticket_id
    #[tokio::test]
    async fn find_textual_object_by_ticket_id_test() {
        // create database
        let random_database = get_random_database().await;

        let pool = connect_to_database(&random_database).await;
        // conn
        let mut conn = pool.acquire().await.unwrap();
        // when result is none
        let textual_object_read = find_to_by_ticket_id(conn.borrow_mut(), &generate_id()).await;
        // delete textual object from database
        assert!(textual_object_read.is_none());

        // when result is one
        let ticket_id = generate_id();
        let to_insert = TextualObject {
            id: Uuid::new_v4(),
            ticket_id: ticket_id.clone(),
            source_id: "source_id".to_string(),
            source_id_type: "test".to_string(),
            source_path: "test".to_string(),
            store_info: "test".to_string(),
            store_url: "test".to_string(),
            source_name: "test".to_string(),
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
            json: sqlx::types::Json(Value::Null),
        };
        let mut conn = pool.acquire().await.unwrap();
        let received_id = insert_to(conn.borrow_mut(), &to_insert).await;
        assert_eq!(received_id, to_insert.id);
        let found_to = find_to_by_ticket_id(conn.borrow_mut(), &ticket_id).await;
        assert!(found_to.is_some());
        assert_eq!(found_to.unwrap().ticket_id, ticket_id);
    }
}
