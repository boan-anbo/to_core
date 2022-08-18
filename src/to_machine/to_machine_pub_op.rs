use crate::enums::store_type::StoreType;
use crate::to::textual_object::TextualObject;
use crate::to::to_dto::{TextualObjectAddManyDto, TextualObjectStoredReceipt};
use crate::to_machine::to_machine_struct::TextualObjectMachine;


//  impl public facing methods for TextualObjectMachine
impl TextualObjectMachine {

    // add from
    pub async fn add_tos(&mut self, add_tos_dto: TextualObjectAddManyDto) -> TextualObjectStoredReceipt {
        // get pool
        let mut pool = self.get_pool().await;

        // create receipt
        let mut receipt = TextualObjectStoredReceipt::from(add_tos_dto.clone());

        receipt.store_info = self.store_info.clone();
        receipt.store_url = self.store_url.clone();

        // iterate over tos IndexMap
        // interate over tos IndexMap asynchronously
        for (ticket_id, add_dto) in add_tos_dto.tos.iter() {
            // convert
            let to = TextualObject::from(add_dto.clone());
            // insert to
            self.add_textual_object(&to).await;
            receipt.tos_stored.insert(ticket_id.clone(), to.clone());
        };


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
            &add_tos_dto.store_dir, StoreType::SQLITE, Some(ToMachineOption{
                use_random_file_name: true,
                ..Default::default()
            })
        ).await;

        // add tos
        let receipt = textual_object_machine.add_tos(add_tos_dto.clone()).await;

        // assert receipt
        assert_eq!(receipt.tos_stored.len(), add_tos_dto.tos.len());
        println!("{:?}", receipt);
    }
}