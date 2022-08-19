use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ToMarker {
    pub left_marker: String,
    pub right_marker: String,
    pub value_entry_separator: String,
    pub value_separator: String,
}

// create default ToMarker
impl Default for ToMarker {
    fn default() -> Self {
        ToMarker {
            left_marker: String::from("[["),
            right_marker: String::from("]]"),
            value_entry_separator: String::from("|"),
            value_separator: String::from(":"),
        }
    }
}