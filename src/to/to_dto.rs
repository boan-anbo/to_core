use std::collections::HashMap;
use chrono::Utc;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::to::to_struct::TextualObject;
use crate::to_card::to_card_struct::TextualObjectCard;
use crate::utils::get_random_test_database_dir::get_random_test_database_dir;
use crate::utils::id_generator::generate_id;
use utoipa::{ToSchema};

#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct TextualObjectStoredReceipt {
    // <Unique_Store_Id, Stored_Textual_Object>
    pub tos_stored: IndexMap<String, TextualObject>,
    // <Unique_Store_Id, Stored_Textual_Object>
    pub store_info: String,
    pub store_url: String,
    pub stored: chrono::NaiveDateTime,
}

// create receipt From TextualObjectStoredReceipt

impl From<TextualObjectAddManyDto> for TextualObjectStoredReceipt {
    fn from(add_tos_dto: TextualObjectAddManyDto) -> Self {
        let mut receipt = TextualObjectStoredReceipt {
            tos_stored: IndexMap::new(),
            store_info: String::new(),
            store_url: add_tos_dto.store_dir,
            stored: Utc::now().naive_utc(),
        };
        if add_tos_dto.store_info.is_some() {
            receipt.store_info = add_tos_dto.store_info.unwrap_or("".to_string());
        };
        receipt
    }
}


#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct TextualObjectAddManyDto {
    // list of textual objects to add, String is the unique source id.
    // this is so that the recept will provide a list of unique ids with stored textual objects.
    pub tos: HashMap<String, TextualObjectAddDto>,
    // whether when there is an existing TO in the store with the same source_id, replace it with the new item.
    pub overwrite: bool,
    pub store_info: Option<String>,
    pub store_dir: String,
    pub store_filename: Option<String>,
}

impl TextualObjectAddManyDto {
    pub fn sample() -> Self {
        let mut sample_dto = TextualObjectAddManyDto {
            tos: HashMap::new(),
            overwrite: false,
            store_info: Some("Random Store Info".to_string()),
            store_dir: get_random_test_database_dir().to_string(),
            store_filename: None,
        };
        for _ in 0..10 {
            sample_dto.tos.insert(generate_id(), TextualObjectAddDto::sample());
        };
        sample_dto
    }
}

#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct TextualObjectAddDto {
    // unique ID of the item in the source
    pub source_id: String,
    pub source_id_type: String,
    // unique path to the object in the source if there is any
    pub source_path: String,
    // name of the source of the textual object, e.g. "Zotero", "DOI"
    pub source_name: String,

    // item
    pub json: serde_json::Value,
}

impl TextualObjectAddDto {
    pub fn sample() -> Self {
        TextualObjectAddDto {
            source_id: "source_id_value".to_string(),
            source_id_type: "source_id_type_value".to_string(),
            source_path: "source_path_value".to_string(),
            source_name: "source_name_value".to_string(),
            json: serde_json::json!({
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
            }),
        }
    }
}

// implement from dto to textual object
impl From<TextualObjectAddDto> for TextualObject {
    fn from(dto: TextualObjectAddDto) -> Self {

        // create a new textual object, ready to persist to the database
        TextualObject {
            id: Uuid::new_v4(),

            source_id: dto.source_id,
            source_id_type: dto.source_id_type,
            source_path: dto.source_path,
            source_name: dto.source_name,

            store_url: String::new(),
            // this will come from the receipt.
            store_info: "".to_string(),

            ticket_id: generate_id(),

            created: Utc::now().naive_utc(),
            updated: Utc::now().naive_utc(),

            card_map: String::new(),
            card: sqlx::types::Json(TextualObjectCard::default()),

            json: sqlx::types::Json(dto.json),
        }
    }
}

// look up dto
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct TextualObjectFindRequestDto {
    pub store_url: String,
    pub ticket_ids: Vec<String>
}

// look up dto
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct TextualObjectFindResultDto {
    pub store_url: String,
    // HashMap<ticket_id, TextualObject>
    pub found_tos: HashMap<String, TextualObject>,
    pub found_tos_count: usize,
    pub missing_tos_ids: Vec<String>,
    pub missing_tos_count: usize,
}
// tests

#[cfg(test)]
mod test {
    use super::*;

    // test from dto to textual object
    #[test]
    fn test_from_dto_to_textual_object() {
        let json_value = serde_json::json!({
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

        let dto = TextualObjectAddDto {
            source_id: "source_id_value".to_string(),
            source_id_type: "source_id_type_value".to_string(),
            source_path: "source_path_value".to_string(),
            source_name: "source_name_value".to_string(),
            json: json_value.clone(),
        };
        let textual_object = TextualObject::from(dto);
        assert_eq!(textual_object.source_id, "source_id_value");
        assert_eq!(textual_object.source_id_type, "source_id_type_value");
        assert_eq!(textual_object.source_path, "source_path_value");
        assert_eq!(textual_object.source_name, "source_name_value");
        assert_eq!(textual_object.store_info, String::from(""));
    }
}