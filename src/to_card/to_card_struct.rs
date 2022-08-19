use chrono::{NaiveDateTime, Utc};
use indexmap::IndexMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub extra: Vec<String>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub person_id: Uuid,
    pub text: String,
    pub extra: Vec<String>,
    pub created: NaiveDateTime,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextualObjectCard {
    id: Uuid,
    // store metadata
    store_id: Uuid,
    store_ticket_id: String,
    store_url: String,
    store_info: String,
    // card data
    title: String,
    description: String,
    creators: IndexMap<usize, Person>,
    comments: Vec<Comment>,
    extra: Vec<String>
}

// implement default
impl Default for TextualObjectCard {
    fn default() -> Self {
        TextualObjectCard {
            id: Uuid::new_v4(),
            store_id: Uuid::new_v4(),
            store_ticket_id: "store_ticket_id_value".to_string(),
            store_url: "store_url_value".to_string(),
            store_info: "store_info_value".to_string(),
            title: "".to_string(),
            description: "".to_string(),
            creators: IndexMap::new(),
            comments: Vec::new(),
            extra: Vec::new()
        }
    }
}

// implement json
impl TextualObjectCard{
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}