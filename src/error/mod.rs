pub mod error_message;

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
// impl default with generic error message
impl ToErrors {
    fn default_with_generic_message(error: ToErrors) -> TextualObjectErrorMessage {
        TextualObjectErrorMessage {
            code: None,
            message: ToErrors::generic_error_message(error),
            suggestion: String::new(),
            payload_for_user: serde_json::Value::Null,
            payload_from_user: serde_json::Value::Null,
        }
    }
}


#[derive(Debug)]
pub enum ToErrors {
    AddManyRequestError(TextualObjectErrorMessage),
    FindRequestError(TextualObjectErrorMessage),
}


impl std::error::Error for ToErrors {}

impl fmt::Display for ToErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ToErrors::AddManyRequestError(message) => write!(f, "{:?}", message),
            ToErrors::FindRequestError(message) => write!(f, "{:?}", message),
        }
    }
}
 impl ToErrors {
     // when there is no need to provide a specific error message, use this function to get a default error message.
     pub fn generic_error_message(error: ToErrors) -> String {
         match error {
             ToErrors::AddManyRequestError(_) => String::from("Add Many Request DTO Error"),
                ToErrors::FindRequestError(_) => String::from("Find Request DTO Error"),
         }
     }
 }

