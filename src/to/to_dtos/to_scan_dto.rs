use std::collections::HashMap;
use crate::to::to_struct::TextualObject;
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};
use crate::error::{TextualObjectErrorMessage, ToErrors};
use crate::error::error_message::ToErrorMessage;
use crate::utils::check_if_file_exists::check_if_file_exists;

/// Dto for scanning TO from text request.
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct ToScanRequestDto {
    pub store_url: String,
    /// the text to scan from
    pub text: String
}

impl ToScanRequestDto {
    pub fn validate(&self) -> Result<(), ToErrors> {
        let mut error_message = TextualObjectErrorMessage::default();
        // check whether ticket ids are provided and whether store_url has file


        if self.text.is_empty() {
            error_message.message = ToErrorMessage::ScanRequestDtoNoText.to_string();
            return Err(ToErrors::FindRequestError(error_message));
        }

        if !check_if_file_exists(&self.store_url) {
            error_message.message = ToErrorMessage::FindOrScanRequestDtoStoreUrlDoesNotExist.to_string();
            return Err(ToErrors::FindRequestError(error_message));
        }

        Ok(())
    }
}

// look up dto
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct ToScanResultDto {
    pub store_url: String,
    // HashMap<ticket_id, TextualObject>
    pub found_tos: Vec<TextualObject>,
    pub found_tos_count: usize,
    pub missing_tos_ids: Vec<String>,
    pub missing_tos_count: usize,
    pub cleaned_text: String,
}

