use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextualObject {
    pub id: Uuid,
    // ticket id
    pub ticket_id: String,
    // unique identifier for the textual object in the original source, e.g. url for webpage, zotero citekey for zotero item, etc, doi for article, etc

    pub source_id: String,
    // name of the source of the textual object, e.g. "Zotero", "DOI"
    pub source_name: String,
    // name of the type of id, e.g. url, Zotero Citekey, DOI, etc.
    pub source_id_type: String,
    // unique path to the textual object, e.g. "/path/to/file.txt". Eg. doi url.
    pub source_path: String,

    // store info, kind of store, JSOn or Sqlite, etc.
    pub store_info: String,
    // store url, e.g. path, or url
    pub store_url: String,

    pub created: chrono::NaiveDateTime,
    pub updated: chrono::NaiveDateTime,

    pub json: sqlx::types::Json< serde_json::Value>
    ,
}
