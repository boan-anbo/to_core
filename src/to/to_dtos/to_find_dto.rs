use std::collections::HashMap;
use crate::to::to_struct::TextualObject;
use utoipa::ToSchema;
use serde::{Deserialize, Serialize};
use crate::error::{TextualObjectErrorMessage, ToErrors};
use crate::error::error_message::ToErrorMessage;
use crate::utils::check_if_file_exists::check_if_file_exists;

// look up dto
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct ToFindRequestDto {
    pub store_url: String,
    // if this is provided, the store_filename and directory will be ignored.
    pub ticket_ids: Vec<String>
}

impl ToFindRequestDto {
    pub fn validate(&self) -> Result<(), ToErrors> {
        let mut error_message = TextualObjectErrorMessage::default();
        // check whether ticket ids are provided and whether store_url has file


        if self.ticket_ids.is_empty() {
            error_message.message = ToErrorMessage::FindRequestDtoNoTicketIds.to_string();
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
pub struct ToFindResultDto {
    pub store_url: String,
    // HashMap<ticket_id, TextualObject>
    pub found_tos: Vec< TextualObject>,
    pub found_tos_count: usize,
    pub missing_tos_ids: Vec<String>,
    pub missing_tos_count: usize,
}

// test
#[cfg(test)]
mod test {
    use super::*;

    // test find request validate
    #[test]
    fn test_find_request_validate() {
        let to_find_request_dto = ToFindRequestDto {
            store_url: "store_url".to_string(),
            ticket_ids: vec!["ticket_id_1".to_string(), "ticket_id_2".to_string()],
        };
        assert!(to_find_request_dto.validate().is_err());
    }
}


