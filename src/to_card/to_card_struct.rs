use chrono::NaiveDateTime;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use strum_macros::{Display, EnumIter, EnumString};

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
pub struct ToCard {
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

#[derive(Debug, Display, EnumString, EnumIter)]
pub enum ToCardField {
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "store_id")]
    StoreId,
    #[strum(serialize = "store_ticket_id")]
    StoreTicketId,
    #[strum(serialize = "store_url")]
    StoreUrl,
    #[strum(serialize = "store_info")]
    StoreInfo,
    #[strum(serialize = "title")]
    Title,
    #[strum(serialize = "description")]
    Description,
    #[strum(serialize = "creators")]
    Creators,
    #[strum(serialize = "comments")]
    Comments,
    #[strum(serialize = "extra")]
    Extra
}

// implement default
impl Default for ToCard {
    fn default() -> Self {
        ToCard {
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
impl ToCard {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}