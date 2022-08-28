use chrono::{FixedOffset, TimeZone, Utc};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::to_card::to_card_struct::ToCard;
use crate::to_ticket::to_ticket_struct::ToTicket;
use crate::to_ticket::to_ticket_utils::print_minimal_ticket;
use crate::utils::id_generator::generate_id;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextualObject {
    pub id: Uuid,
    // ticket id
    pub ticket_id: String,
    // unique identifier for the textual object in the original source, e.g. url for webpage, zotero citekey for zotero item, etc, doi for article, etc
    pub ticket_minimal: String,

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

    pub json: sqlx::types::Json<serde_json::Value>,

    pub card: sqlx::types::Json<ToCard>,

    // map of string to string, format: "to_key1, card_key1; to_key2, card_key2;" etc.
    pub card_map: String,
}

// implement default values for textual object
impl Default for TextualObject {
    fn default() -> Self {
        let ticket_id = generate_id();
        TextualObject {
            id: Uuid::new_v4(),
            ticket_id: ticket_id.clone(),
            ticket_minimal: print_minimal_ticket(&ticket_id, None),
            source_id: String::new(),
            source_name: String::new(),
            source_id_type: String::new(),
            source_path: String::new(),
            store_info: String::new(),
            store_url: String::new(),
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
            json: sqlx::types::Json(serde_json::Value::Null),
            card: sqlx::types::Json(ToCard::default()),
            card_map: String::new(),
        }
    }
}


// default with uuid
impl TextualObject {
    pub fn default_with_uuid(uuid: Uuid) -> Self {
        let ticket_id = generate_id();
        TextualObject {
            id: uuid,
            ticket_id: ticket_id.clone(),
            ticket_minimal: print_minimal_ticket(&ticket_id, None),
            source_id: String::new(),
            source_name: String::new(),
            source_id_type: String::new(),
            source_path: String::new(),
            store_info: String::new(),
            store_url: String::new(),
            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),
            json: sqlx::types::Json(serde_json::Value::Null),
            card: sqlx::types::Json(ToCard::default()),
            card_map: String::new(),
        }
    }
}

// implement a factory method to create sample textual object for testing and seeding the database
impl TextualObject {
    pub fn get_sample() -> TextualObject {
        let _json = serde_json::json!({
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
        });
        let mut to = TextualObject::default();
        to.json = sqlx::types::Json(_json);
        to

    }
}

// implement converter from textual object to TextualObjectTicket
impl From<TextualObject> for ToTicket {
    fn from(textual_object: TextualObject) -> ToTicket {
        // convert textual_object.json to IndexMap
        let json = &textual_object.json.0;
        let mut index_map: IndexMap<String, String> = IndexMap::new();
        for (key, value) in json.as_object().unwrap().iter() {
            index_map.insert(key.to_string(), value.to_string());
        }

        // if length > 0, then assign the value
        let store_url = if !textual_object.store_url.is_empty()  {
            Some(textual_object.store_url.clone())
        } else {
            Some(String::new())
        };
        let store_info = if !textual_object.store_info.is_empty() {
            Some(textual_object.store_info.clone())
        } else {
            Some(String::new())
        };
        ToTicket {
            id: String::new(),
            ticket_id: textual_object.ticket_id.clone(),
            values: index_map,
            to_updated: FixedOffset::east(0).timestamp(textual_object.updated.timestamp(), 0),
            to_store_url: store_url,
            to_store_info: store_info,
            to_marker: Default::default(),
            to_intext_option: None,
        }
    }
}

// test module
#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::to_ticket::to_ticket_struct::ToTicket;

    // test get_sample
    #[test]
    fn get_sample_test() {
        let textual_object = super::TextualObject::get_sample();
        assert!(textual_object.id != Uuid::new_v4());
    }

    // test textual_object to textual_object_ticket
    #[test]
    fn textual_object_from_textual_object_ticket_test() {
        let sample_textual_object = super::TextualObject::get_sample();
        let textual_object_ticket = ToTicket::from(sample_textual_object.clone());
        assert_eq!(&textual_object_ticket.ticket_id, &sample_textual_object.ticket_id);
        assert_eq!(textual_object_ticket.to_store_info.as_ref().unwrap(), &sample_textual_object.store_info);
        assert_eq!(textual_object_ticket.to_store_url.as_ref().unwrap(), &sample_textual_object.store_url);


        let ticket = textual_object_ticket.print(None);
        assert!(ticket.len() > 0);

    }
}
