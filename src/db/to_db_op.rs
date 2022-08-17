use sqlx::{Error, Pool, Row, Sqlite};
use sqlx::sqlite::{SqliteQueryResult, SqliteRow};
use uuid::Uuid;
use crate::db::db_op::reset_database;
use crate::to::textual_object::TextualObject;

// store textual object into database
pub(crate) async fn insert_to(pool: &Pool<Sqlite>, textual_object: &TextualObject) -> Uuid {
    let id = textual_object.id.to_string();
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
        json)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
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
        textual_object.json
    );
    insert_query.execute(pool).await.unwrap();
    textual_object.id

}

// read textual object from database
pub(crate) async fn find_to_by_id(pool: &Pool<Sqlite>, id: &Uuid) -> Option<TextualObject> {
    let textual_object_row = sqlx::query(
        "SELECT * FROM textual_objects WHERE id = $1",
    )
        .bind(id)
        .fetch_one(pool)
        .await;
    load_sqlite_row_to_textual_object(textual_object_row)
}

// find to by ticket id
pub(crate) async fn find_to_by_ticket_id(pool: &Pool<Sqlite>, ticket_id: &str) -> Option<TextualObject> {
    let textual_object_rows = sqlx::query(
        "SELECT * FROM textual_objects WHERE ticket_id = $1",
    )
        .bind(ticket_id)
        .fetch_one(pool)
        .await;
    load_sqlite_row_to_textual_object(textual_object_rows)
}


// count the number of textual objects in the database
pub(crate) async fn count_textual_objects(pool: &Pool<Sqlite>) -> i64 {
    let count = sqlx::query("SELECT COUNT(*) FROM textual_objects")
        .fetch_one(pool)
        .await
        .unwrap()
        .get(0);
    count
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
            Some(textual_object)
        }
        Err(e) => {
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
pub(crate) async fn delete_to_by_ticket_id(pool: &Pool<Sqlite>, ticket_id: &String) -> SqliteQueryResult {
    let delete_query = sqlx::query("DELETE FROM textual_objects WHERE ticket_id = $1")
        .bind(ticket_id)
        .execute(pool)
        .await
        .unwrap();
    delete_query
}
