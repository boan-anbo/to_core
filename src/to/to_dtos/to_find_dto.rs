use std::collections::HashMap;
use crate::to::to_struct::TextualObject;
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};

// look up dto
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct TextualObjectFindRequestDto {
    pub store_url: String,
    // if this is provided, the store_filename and directory will be ignored.
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


