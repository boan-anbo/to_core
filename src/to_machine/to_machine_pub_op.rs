use crate::enums::store_type::StoreType;
use crate::to::to_struct::TextualObject;
use crate::to::to_dto::{TextualObjectAddManyDto, TextualObjectStoredReceipt};
use crate::to_machine::to_machine_struct::TextualObjectMachine;


/*
These are methods mostly exposed to the ToApi, such batch adding dtos etc--why it's called public operation methods
 */
impl TextualObjectMachine {
    // add from
    pub async fn add_tos(&mut self, add_tos_dto: TextualObjectAddManyDto) -> TextualObjectStoredReceipt {
        // get pool
        let mut pool = self.get_pool().await;

        // create receipt
        let mut receipt = TextualObjectStoredReceipt::from(add_tos_dto.clone());


        // iterate over tos IndexMap
        // interate over tos IndexMap asynchronously
        for (source_id, add_dto) in add_tos_dto.tos.iter() {
            // convert
            let mut to = TextualObject::from(add_dto.clone());

            // generate tha assign ticket id to the TO to be added
            let unique_ticket_id = self.get_unique_ticket_id().await;
            to.ticket_id = unique_ticket_id.clone();

            // save store info to to
            to.store_info = self.store_info.clone();
            to.store_url = self.store_url.clone();
            to.source_id = source_id.clone();
            // insert to
            self.add_textual_object(&to).await;
            receipt.tos_stored.insert(unique_ticket_id, to);
        };

        // save metadata to receipt
        receipt.store_info = self.store_info.clone();
        receipt.store_url = self.store_url.clone();
        receipt
    }
}

// test
#[cfg(test)]
mod test {
    use crate::enums::store_type::StoreType;
    use crate::to::to_dto::TextualObjectAddManyDto;
    use crate::to_machine::to_machine_option::ToMachineOption;
    use crate::to_machine::to_machine_struct::TextualObjectMachine;
    use crate::utils::get_random_test_database_dir::get_random_test_database_dir;

    // test add_tos
    #[tokio::test]
    async fn test_add_tos() {
        //
        // create add_tos_dto
        let mut add_tos_dto = TextualObjectAddManyDto::sample();

        // get resources test folder


        // create TextualObjectMachine
        let mut textual_object_machine = TextualObjectMachine::new(
            &add_tos_dto.store_dir, StoreType::SQLITE, Some(ToMachineOption {
                use_random_file_name: true,
                store_info: Some("Random Store Info".to_string()),
                store_file_name: add_tos_dto.store_filename.clone(),
                ..Default::default()
            }),
        ).await;

        // add tos
        let receipt = textual_object_machine.add_tos(add_tos_dto.clone()).await;

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

        println!("{:?}", serde_json::to_string_pretty(&receipt).unwrap());
    }
}