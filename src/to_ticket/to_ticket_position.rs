use regex::Captures;
use serde::{Deserialize, Serialize};

/// struct for recognition the regex match position: line, column, length of the ticket in the original text
#[derive(Serialize, Deserialize, Clone)]
pub struct ToTicketPositionInfo {
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub raw_text: String,
}

impl ToTicketPositionInfo {
    pub fn from_match(m: &Captures, line: usize) -> Self {
        ToTicketPositionInfo {
            line,
            column: m.get(0).unwrap().start(),
            length: m.get(0).unwrap().end() - m.get(0).unwrap().start(),
            raw_text: m.get(0).unwrap().as_str().to_string(),
        }
    }
}