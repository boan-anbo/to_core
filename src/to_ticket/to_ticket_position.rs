use regex::{Captures};
use serde::{Deserialize, Serialize};

// struct for recognition the regex match position: line, column, length
#[derive(Serialize, Deserialize)]
pub struct ToTicketInTextPosition {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl ToTicketInTextPosition {
    pub fn from_match(m: &Captures, line: usize) -> Self {
        ToTicketInTextPosition {
            line,
            column: m.get(0).unwrap().start(),
            length: m.get(0).unwrap().end() - m.get(0).unwrap().start(),
        }
    }
}