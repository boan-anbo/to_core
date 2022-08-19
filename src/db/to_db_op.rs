use std::borrow::BorrowMut;

use sqlx::{Error, Pool, Row, Sqlite};
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteQueryResult, SqliteRow};
use uuid::Uuid;

use crate::to::to_struct::TextualObject;

// store textual object into database
pub(crate) async fn insert_to(pool: &mut PoolConnection<Sqlite>, textual_object: &TextualObject) -> Uuid {
    let _id = textual_object.id.to_string();
    // insert textual object into database
    let insert_query = sqlx::query!(
        "INSERT INTO textual_objects (id,
        ticket_id,
        source_id,
        source_name,
        source_id_type,
        source_path,
        store_info,
        store_url,
        created,
        updated,
        json,
        card,
        card_map,
        ticket_minimal
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
        textual_object.id,
        textual_object.ticket_id,
        textual_object.source_id,
        textual_object.source_name,
        textual_object.source_id_type,
        textual_object.source_path,
        textual_object.store_info,
        textual_object.store_url,
        textual_object.created,
        textual_object.updated,
        textual_object.json,
        textual_object.card,
        textual_object.card_map,
        textual_object.ticket_minimal,
    );
    insert_query.execute(pool).await.unwrap();
    textual_object.id
}

// read textual object from database
pub(crate) async fn find_to_by_id(conn: &mut PoolConnection<Sqlite>, id: &Uuid) -> Option<TextualObject> {
    let textual_object_row = sqlx::query(
        "SELECT * FROM textual_objects WHERE id = $1",
    )
        .bind(id)
        .fetch_one(conn)
        .await;
    load_sqlite_row_to_textual_object(textual_object_row)
}

// find to by ticket id
pub(crate) async fn find_to_by_ticket_id(pool: &mut PoolConnection<Sqlite>, ticket_id: &str) -> Option<TextualObject> {
    let textual_object_rows = sqlx::query(
        "SELECT * FROM textual_objects WHERE ticket_id = $1",
    )
        .bind(ticket_id)
        .fetch_one(pool)
        .await;
    load_sqlite_row_to_textual_object(textual_object_rows)
}

// check if there is any row with the given ticket id
pub(crate) async fn check_if_ticket_id_exists(pool: &mut PoolConnection<Sqlite>, ticket_id: &str) -> bool {
    let textual_object_rows = sqlx::query(
        "SELECT * FROM textual_objects WHERE ticket_id = $1",
    )
        .bind(ticket_id)
        .fetch_one(pool)
        .await;
    match textual_object_rows {
        Ok(_s) => true,
        Err(_e) => false,
    }
}

// count the number of textual objects in the database
pub(crate) async fn count_textual_objects(mut pool: PoolConnection<Sqlite>) -> i64 {
    let count_query = sqlx::query("SELECT COUNT(*) FROM textual_objects");
    let count = count_query.fetch_one(pool.borrow_mut()).await.unwrap();
    count.get(0)
}

// load multiple sqlite rows to textual objecs
fn load_multiple_sqlite_rows_to_textual_objects(textual_object_rows: Result<Vec<SqliteRow>, Error>) -> Vec<TextualObject> {
    let mut textual_objects = Vec::new();
    match textual_object_rows {
        Ok(textual_object_rows) => {
            for textual_object_row in textual_object_rows {
                let textual_object = load_sqlite_row_to_textual_object(Ok(textual_object_row));
                match textual_object {
                    Some(textual_object) => {
                        textual_objects.push(textual_object);
                    }
                    None => {
                        // quiet throw
                        panic!("{:?}", "textual object not found");
                    }
                }
            }
        }
        Err(e) => {
            // quiet throw
            panic!("{:?}", e);
        }
    }
    textual_objects
}

// utility function to load sqlite_row results into textual object
fn load_sqlite_row_to_textual_object(textual_object_row: Result<SqliteRow, Error>) -> Option<TextualObject> {
    match textual_object_row {
        Ok(textual_object_row) => {
            let textual_object = TextualObject {
                id: textual_object_row.get("id"),
                ticket_id: textual_object_row.get("ticket_id"),
                ticket_minimal: textual_object_row.get("ticket_minimal"),
                source_id: textual_object_row.get("source_id"),
                source_id_type: textual_object_row.get("source_id_type"),
                source_path: textual_object_row.get("source_path"),
                store_info: textual_object_row.get("store_info"),
                store_url: textual_object_row.get("store_url"),
                source_name: textual_object_row.get("source_name"),
                created: textual_object_row.get("created"),
                updated: textual_object_row.get("updated"),
                json: textual_object_row.get("json"),
                card: textual_object_row.get("card"),
                card_map: textual_object_row.get("card_map"),
            };

            Some(textual_object)
        }
        Err(_e) => {
            None
        }
    }
}


// delete textual object from database by id
pub(crate) async fn delete_to_by_id(pool: &Pool<Sqlite>, id: &Uuid) -> SqliteQueryResult {
    let delete_query = sqlx::query("DELETE FROM textual_objects WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    delete_query
}

// delete textual object from database by ticket id
pub(crate) async fn delete_to_by_ticket_id(pool: &mut PoolConnection<Sqlite>, ticket_id: &String) -> SqliteQueryResult {
    let delete_query = sqlx::query("DELETE FROM textual_objects WHERE ticket_id = $1")
        .bind(ticket_id)
        .execute(pool)
        .await
        .unwrap();
    delete_query
}
