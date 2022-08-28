use std::fmt;
use serde_json::Value;
use serde::{Deserialize, Serialize};

// message: String, suggestion: String, payload: Value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextualObjectErrorMessage {
    pub code: Option<usize>,
    pub message: String,
    pub suggestion: String,
    pub payload_for_user: Value,
    pub payload_from_user: Value,
}

// impl default
impl Default for TextualObjectErrorMessage {
    fn default() -> Self {
        TextualObjectErrorMessage {
            code: None,
            message: String::new(),
            suggestion: String::new(),
            payload_for_user: serde_json::Value::Null,
            payload_from_user: serde_json::Value::Null,
        }
    }
}


#[derive(Debug)]
pub enum TextualObjectErrors {
    AddManyRequestError(TextualObjectErrorMessage),
}

impl std::error::Error for TextualObjectErrors {}

impl fmt::Display for TextualObjectErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextualObjectErrors::AddManyRequestError(message) => write!(f, "{:?}", message),
        }
    }
}

