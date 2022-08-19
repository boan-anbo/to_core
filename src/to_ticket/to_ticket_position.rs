use regex::Captures;
use serde::{Deserialize, Serialize};

// struct for recognition the regex match position: line, column, length
#[derive(Serialize, Deserialize, Clone)]
pub struct ToTicketInTextInfo {
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub raw_text: String,
}

impl ToTicketInTextInfo {
    pub fn from_match(m: &Captures, line: usize) -> Self {
        ToTicketInTextInfo {
            line,
            column: m.get(0).unwrap().start(),
            length: m.get(0).unwrap().end() - m.get(0).unwrap().start(),
            raw_text: m.get(0).unwrap().as_str().to_string(),
        }
    }
}