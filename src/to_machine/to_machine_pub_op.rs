use std::collections::HashMap;
use crate::error::ToErrors;

use crate::to::to_dtos::to_add_dto::{ToAddManyDto, TextualObjectStoredReceipt};
use crate::to::to_dtos::to_find_dto::{ToFindRequestDto, ToFindResultDto};
use crate::to::to_dtos::to_scan_dto::{ToScanRequestDto, ToScanResultDto};
use crate::to::to_struct::TextualObject;
use crate::to_machine::to_machine_struct::ToMachine;
use crate::to_parser::parser::ToParser;
use crate::to_parser::parser_option::ToParserOption;
use crate::to_ticket::to_ticket_struct::ToTicket;

/// These are methods mostly exposed to the ToApi, such batch adding dtos etc--why it's called public operation methods
///
impl ToMachine {
    /// add from TextualObjectAddManyDto, main method for adding from dto
    pub async fn add_tos(&mut self, add_tos_dto: ToAddManyDto) -> Result<TextualObjectStoredReceipt, ToErrors> {

        // validate dto
        let is_valid = add_tos_dto.is_valid();
        match is_valid {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        // get pool
        let _pool = self.get_pool().await;

        // create receipt
        let mut receipt = TextualObjectStoredReceipt::from(add_tos_dto.clone());


        // iterate over tos IndexMap
        // interate over tos IndexMap asynchronously
        for to_to_add in add_tos_dto.tos.iter() {
            // convert
            let mut to = TextualObject::from(to_to_add.clone());

            // generate tha assign ticket id to the TO to be added
            let unique_ticket_id = self.get_unique_ticket_id().await;
            to.ticket_id = unique_ticket_id.clone();

            // save store info to to
            to.store_info = self.store_info.clone();
            to.store_url = self.store_url.clone();
            to.source_id = String::from(&to_to_add.source_id.clone().unwrap_or("".to_string()));
            // insert to
            self.add_textual_object(&to).await;
            receipt.tos_stored.insert(unique_ticket_id, to);
            receipt.total_tos_stored += 1;
        };

        // save metadata to receipt
        receipt.store_info = self.store_info.clone();
        receipt.store_url = self.store_url.clone();
        Ok(receipt)
    }

    /// find TOs by ticket ids
    pub async fn find_tos_by_ticket_ids(&mut self, find_request_dto: &ToFindRequestDto) -> Result<ToFindResultDto, ToErrors> {
        // validate dto
        let is_valid = find_request_dto.validate();
        match is_valid {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }
        // find by ticket ids
        let (found_tos, missing_to_ids) = self.find_by_ticket_ids(&find_request_dto.ticket_ids).await;
        let result = ToFindResultDto {
            found_tos_count: found_tos.len(),
            missing_tos_count: missing_to_ids.len(),
            found_tos,
            missing_tos_ids: missing_to_ids,
            store_url: self.store_url.clone(),
        };
        Ok(result)
    }

    /// This is higher level than find_tos_by_ticket_ids, for it classify the results into found and missing
    ///
    async fn find_by_ticket_ids(&mut self, ticket_ids: &Vec<String>) -> (Vec<TextualObject>, Vec<String>) {
// use find method to get all tos
        let mut found_tos: Vec<TextualObject> = Vec::new();
        let mut missing_to_ids: Vec<String> = vec![];
        for ticket_id in ticket_ids.iter() {
            let found_to = self.find(ticket_id).await;
            match found_to {
                Some(found_to) => {
                    found_tos.push(found_to);
                }
                None => {
                    missing_to_ids.push(ticket_id.clone());
                }
            }
        };
        (found_tos, missing_to_ids)
    }

    // find TOs by text
    pub async fn find_tos_by_text(&mut self, scan_request: &ToScanRequestDto) -> Result<ToScanResultDto, ToErrors> {
        let validation = scan_request.validate();
        match validation {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }
        // use find method to get all tos
        let matched_to_tickets = ToParser::scan_text_for_tickets(&scan_request.text, ToParserOption::default());

        let found_tos = self.find_by_ticket_ids(&matched_to_tickets.iter().map(
            |ticket_id| ticket_id.ticket_id.to_string()
        ).collect()).await;


        let result = ToScanResultDto {
            found_tos_count: found_tos.0.len(),
            missing_tos_count: found_tos.1.len(),
            found_tos: found_tos.0,
            missing_tos_ids: found_tos.1,
            store_url: self.store_url.clone(),
            cleaned_text: scan_request.text.clone(),
        };

        Ok(result)

    }

}

// test
#[cfg(test)]
mod test {
    use std::fmt::Debug;
    use crate::db::db_op::join_db_path;
    use crate::enums::store_type::StoreType;
    use crate::error::error_message::ToErrorMessage;
    use crate::error::ToErrors;
    use crate::to::to_dtos::to_add_dto::ToAddManyDto;
    use crate::to::to_dtos::to_find_dto::ToFindRequestDto;
    use crate::to::to_struct::TextualObject;
    use crate::to_machine::to_machine_option::ToMachineOption;
    use crate::to_machine::to_machine_struct::ToMachine;
    use crate::utils::get_random_test_database_dir::get_random_test_database_dir;
    use crate::utils::id_generator::generate_id;

    // test add_tos
    #[tokio::test]
    async fn test_add_tos() {
        //
        // create add_tos_dto
        let add_tos_dto = ToAddManyDto::sample();

        // get resources test folder


        // create TextualObjectMachine
        let mut textual_object_machine = ToMachine::new(
            &add_tos_dto.store_dir, StoreType::SQLITE, Some(ToMachineOption {
                use_random_file_name: true,
                store_info: Some("Random Store Info".to_string()),
                store_file_name: add_tos_dto.store_filename.clone(),
                ..Default::default()
            }),
        ).await;

        // add tos
        let result = textual_object_machine.add_tos(add_tos_dto.clone()).await;

        let receipt = match result {
            Ok(receipt) => receipt,
            Err(e) => {
                panic!("{:?}", e);
            }
        };

        // assert receipt
        assert_eq!(receipt.tos_stored.len(), add_tos_dto.tos.len());
        // first stored sto
        let first_key = receipt.tos_stored.keys().next().unwrap();
        let first_stored_to = receipt.tos_stored.get(first_key).unwrap();
        // check key and ticket_id
        assert_eq!(first_key, &first_stored_to.ticket_id);
        // check stored to has store information and ticket id
        assert_eq!(first_stored_to.store_url, textual_object_machine.store_url);
        assert_eq!(first_stored_to.store_info, textual_object_machine.store_info);

        // check tos count
        assert_eq!(receipt.total_tos_stored, add_tos_dto.tos.len());

        println!("{:?}", serde_json::to_string_pretty(&receipt).unwrap());
    }

    // should throw when add_tos request is invalid
    #[tokio::test]
    async fn test_add_tos_invalid() {
        //
        // create add_tos_dto
        let add_tos_dto = ToAddManyDto::sample();
        let mut invalid_add_tos_dto = add_tos_dto.clone();
        invalid_add_tos_dto.tos.clear();
        // get resources test folder
        let store_dir = get_random_test_database_dir();
        // create TextualObjectMachine
        let mut textual_object_machine = ToMachine::new(
            &store_dir, StoreType::SQLITE, Some(ToMachineOption {
                use_random_file_name: true,
                store_info: Some("Random Store Info".to_string()),
                store_file_name: add_tos_dto.store_filename.clone(),
                ..Default::default()
            }),
        ).await;

        // add tos
        let result = textual_object_machine.add_tos(invalid_add_tos_dto.clone()).await;
        let error = match result {
            Ok(_) => {
                panic!("Expected error");
            }
            Err(e) => e,
        };

        let add_error = match error {
            ToErrors::AddManyRequestError(add_error) => add_error,
            e => {
                panic!("Expected invalid request error");
            }
        };
        // assert error
        assert_eq!(add_error.code, None);
        assert_eq!(add_error.message, "No textual objects to add");
    }

    // test find_tos_by_ticket_ids
    #[tokio::test]
    async fn test_find_tos_by_ticket_ids() {
        //
        let to_1 = TextualObject::get_sample();
        let to_2 = TextualObject::get_sample();
        let to_3 = TextualObject::get_sample();

        // get resources test folder
        let test_database_dir = get_random_test_database_dir();
        // create add_tos_dto
        // random filename
        let random_filename = generate_id();

        let find_request_dto = ToFindRequestDto {
            ticket_ids: vec![to_1.ticket_id.clone(), to_2.ticket_id.clone(), to_3.ticket_id.clone()],
            store_url: join_db_path(&test_database_dir, &random_filename).to_string(),
        };
        // create TextualObjectMachine
        let mut textual_object_machine = ToMachine::new(
            &test_database_dir, StoreType::SQLITE, Some(ToMachineOption {
                store_file_name: Some(random_filename.clone()),
                ..Default::default()
            }),
        ).await;
        // search
        let result_missing_wrapped = textual_object_machine.find_tos_by_ticket_ids(&find_request_dto).await;
        let result_missing = result_missing_wrapped.unwrap();
// assert result
        assert_eq!(result_missing.missing_tos_ids.len(), 3);
        // add one
        textual_object_machine.add_textual_object(&to_1).await;
        // search again
        let result_found_one_wrapped = textual_object_machine.find_tos_by_ticket_ids(&find_request_dto).await;
        let result_found_one = result_found_one_wrapped.unwrap();
        // assert result
        assert_eq!(result_found_one.missing_tos_ids.len(), 2);
        assert_eq!(result_found_one.found_tos.len(), 1);
        assert_eq!(result_found_one.found_tos_count, 1);
        assert_eq!(result_found_one.missing_tos_count, 2);
        let first_found = result_found_one.found_tos.get(&to_1.ticket_id).unwrap();
        assert_eq!(first_found.ticket_id, to_1.ticket_id);
        // add three
        textual_object_machine.add_textual_object(&to_2).await;
        textual_object_machine.add_textual_object(&to_3).await;
        let first_found_to = result_found_one.found_tos.get(&to_1.ticket_id).unwrap();
        assert_eq!(first_found_to.ticket_id, to_1.ticket_id);
        // search again
        let result_found_all_wrapped = textual_object_machine.find_tos_by_ticket_ids(&find_request_dto).await;
        let result_found_all = result_found_all_wrapped.unwrap();
        // assert result
        assert_eq!(result_found_all.missing_tos_ids.len(), 0);
        assert_eq!(result_found_all.found_tos.len(), 3);
        // check found tos
        let first_found_to = result_found_all.found_tos.get(&to_1.ticket_id).unwrap();
        assert_eq!(first_found_to.ticket_id, to_1.ticket_id);
        let second_found_to = result_found_all.found_tos.get(&to_2.ticket_id).unwrap();
        assert_eq!(second_found_to.ticket_id, to_2.ticket_id);
        let third_found_to = result_found_all.found_tos.get(&to_3.ticket_id).unwrap();
        assert_eq!(third_found_to.ticket_id, to_3.ticket_id);

        // check result store url equals to machine store url
        assert_eq!(result_found_all.store_url, textual_object_machine.store_url);
    }
    // test find request dto with invalid request
    #[tokio::test]
    async fn test_find_tos_by_ticket_ids_invalid() {
        //

        // get resources test folder
        let test_database_dir = get_random_test_database_dir();
        // create add_tos_dto
        // random filename
        let random_filename = generate_id();

        let find_request_dto = ToFindRequestDto {
            ticket_ids: vec![],
            store_url: join_db_path(&test_database_dir, &random_filename).to_string(),
        };
        // create TextualObjectMachine
        let mut textual_object_machine = ToMachine::new(
            &test_database_dir, StoreType::SQLITE, Some(ToMachineOption {
                store_file_name: Some(random_filename.clone()),
                ..Default::default()
            }),
        ).await;
        // search
        let result_missing_wrapped = textual_object_machine.find_tos_by_ticket_ids(&find_request_dto).await;
        match result_missing_wrapped {
            Ok(_) => {
                panic!("Expected error");
            }
            Err(e) => {
                if let ToErrors::FindRequestError(e) = e {
                    assert_eq!(e.message, ToErrorMessage::FindRequestDtoNoTicketIds.to_string());
                }
            }
        }
    }

    // Todo, write test for scan request
    #[tokio::test]
    async fn test_scan_request() {
        
    }
}