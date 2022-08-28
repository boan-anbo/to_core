// implement data operation methods for TextualObjectMachine

use std::borrow::BorrowMut;

use uuid::Uuid;

use crate::db::to_db_op::{check_if_ticket_id_exists, count_textual_objects, delete_to_by_ticket_id, find_to_by_ticket_id, insert_to};
use crate::to::to_struct::TextualObject;
use crate::to_machine::to_machine_struct::ToMachine;
use crate::utils::id_generator::generate_id;

impl ToMachine {
    pub async fn update_to_count(&mut self) -> i64 {
        let pool = self.get_pool().await;

        let count = count_textual_objects(pool).await;
        self.set_to_count(count);
        self.to_count
    }

    // add from 

    pub async fn add_textual_object(&mut self, textual_object: &TextualObject) -> Uuid {
        let mut pool = self.get_pool().await;
        let id = insert_to(pool.borrow_mut(), textual_object).await;
        // update to_count
        self.update_to_count().await;
        id
    }

    // find by ticket id
    pub async fn find(&mut self, ticket_id: &str) -> Option<TextualObject> {
        let found_to = find_to_by_ticket_id(self.get_pool().await.borrow_mut(), ticket_id).await;
        found_to
    }

    // find all by ticket ids
    pub async fn find_all(&mut self, ticket_ids: &Vec<&str>) -> Vec<TextualObject> {
        // use find method to get all tos
        let mut found_tos = Vec::new();
        for ticket_id in ticket_ids {
            let found_to = self.find(ticket_id).await;
            match found_to {
                Some(found_to) => {
                    found_tos.push(found_to);
                }
                None => {
                    // do nothing
                }
            }
        }
        found_tos
    }

    // delete by ticket id, return true if successful
    pub async fn delete(&mut self, ticket_id: &String) -> bool {
        let mut pool = self.get_pool().await;
        let result = delete_to_by_ticket_id(pool.borrow_mut(), ticket_id).await;
        // update to_count
        self.update_to_count().await;
        if result.rows_affected() == 1 {
            true
        } else {
            false
        }
    }

    pub async fn get_unique_ticket_id(&mut self) -> String {
        let mut unique_ticket_id_to_try = generate_id();
        let mut pool = self.get_pool().await;
        while check_if_ticket_id_exists(pool.borrow_mut(), unique_ticket_id_to_try.as_str()).await {
            unique_ticket_id_to_try = generate_id();
        }
        unique_ticket_id_to_try
    }
}

// tests for TextualObjectMachine operation methods
#[cfg(test)]
mod test {

    // initiate for tests

    use std::path::PathBuf;

    use crate::enums::store_type::StoreType;
    use crate::to::to_struct::TextualObject;
    use crate::to_machine::to_machine_option::ToMachineOption;
    use crate::to_machine::to_machine_struct::ToMachine;
    use crate::utils::get_random_test_database_dir::get_random_test_database_dir;
    use crate::utils::id_generator::generate_id;

    pub fn get_test_asset_path(file_name: &str) -> String {
        let mut cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cargo_dir.push("resources/test/");
        cargo_dir.push(file_name);
        // convert the PathBuf to path string
        cargo_dir.into_os_string().into_string().unwrap()
    }

    pub fn get_test_asset_path_without_file() -> String {
        let mut cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cargo_dir.push("resources/test/db/");
        // convert the PathBuf to path string
        cargo_dir.into_os_string().into_string().unwrap()
    }

    pub fn get_test_asset_path_with_default_name() -> String {
        let mut cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cargo_dir.push("resources/test/");
        cargo_dir.push("_to_store.db");
        // convert the PathBuf to path string
        cargo_dir.into_os_string().into_string().unwrap()
    }

    // test add to to existent sqlite
    #[tokio::test]
    async fn test_new_existent_sqlite_add() {
        let existent_sqlite_file = get_test_asset_path_without_file();
        // create a new TextualObjectMachineRs with SQLITE store
        let mut tom = ToMachine::new(&existent_sqlite_file, StoreType::SQLITE, Some(
            ToMachineOption {
                use_random_file_name: true,
                ..Default::default()
            }
        )).await;
        let current_to_count = tom.to_count;
        // check if the machine is created
        assert_eq!(tom.store_type, StoreType::SQLITE);
        // create a new textual object
        let sample_to = TextualObject::get_sample();
        // add the textual object to the machine
        let _id = tom.add_textual_object(&sample_to).await;
        // check if the textual object is added
        assert_eq!(tom.to_count, current_to_count + 1);
        tom.delete_store().await;
    }

    // test find by ticket id
    #[tokio::test]
    async fn test_find_by_ticket_id() {
        let existent_sqlite_file = get_test_asset_path_without_file();
        // create a new TextualObjectMachineRs with SQLITE store
        let mut tom = ToMachine::new(&existent_sqlite_file, StoreType::SQLITE, Some(ToMachineOption {
            store_file_name: Some(generate_id()),
            ..Default::default()
        })).await;
        let current_to_count = tom.to_count;
        // check if the machine is created
        assert_eq!(tom.store_type, StoreType::SQLITE);
        // create a new textual object
        let sample_to = TextualObject::get_sample();
        // add the textual object to the machine
        let _id = tom.add_textual_object(&sample_to).await;
        // check if the textual object is added
        assert_eq!(tom.to_count, current_to_count + 1);
        // find the textual object by ticket id
        let found_to = tom.find(&sample_to.ticket_id).await;
        // check if the textual object is found
        assert_eq!(found_to.unwrap().ticket_id, sample_to.ticket_id);
    }

    // test delete by ticket id
    #[tokio::test]
    async fn test_delete_by_ticket_id() {
        let existent_sqlite_file = get_test_asset_path_without_file();
        // create a new TextualObjectMachineRs with SQLITE store
        let mut tom = ToMachine::new(&existent_sqlite_file, StoreType::SQLITE,
                                     Some(ToMachineOption {
                                                    store_file_name: Some(generate_id()),
                                                    ..Default::default()
                                                }),
        ).await;
        let current_to_count = tom.to_count;
        // check if the machine is created
        assert_eq!(tom.store_type, StoreType::SQLITE);
        // create a new textual object
        let sample_to = TextualObject::get_sample();
        // add the textual object to the machine
        let _id = tom.add_textual_object(&sample_to).await;
        // check if the textual object is added
        assert_eq!(tom.to_count, current_to_count + 1);
        // find the textual object by ticket id
        let found_to = tom.find(&sample_to.ticket_id).await;
        // check if the textual object is found
        assert_eq!(found_to.unwrap().ticket_id, sample_to.ticket_id);
        // delete the textual object by ticket id
        let result = tom.delete(&sample_to.ticket_id).await;
        // check if the textual object is deleted
        assert_eq!(result, true);
        // check if the textual object is not found
        let found_to = tom.find(&sample_to.ticket_id).await;
        assert_eq!(found_to.is_none(), true);
        // check count after delete
        assert_eq!(tom.to_count, current_to_count);
    }

    // test find all by ticket ids
    #[tokio::test]
    async fn test_find_all_by_ticket_ids() {
        let existent_sqlite_file = get_test_asset_path_without_file();
        // create a new TextualObjectMachineRs with SQLITE store
        let mut tom = ToMachine::new(&existent_sqlite_file, StoreType::SQLITE, None).await;
        let _current_to_count = tom.to_count;
        // create three new textual objects
        let sample_to1 = TextualObject::get_sample();
        let sample_to2 = TextualObject::get_sample();
        let sample_to3 = TextualObject::get_sample();
        // add the textual objects to the machine
        let _id1 = tom.add_textual_object(&sample_to1).await;
        let _id2 = tom.add_textual_object(&sample_to2).await;
        let _id3 = tom.add_textual_object(&sample_to3).await;
        // find all the textual objects by ticket ids
        let found_tos = tom.find_all(&vec![&sample_to1.ticket_id, &sample_to2.ticket_id, &sample_to3.ticket_id]).await;
        // check if the textual objects are found
        assert_eq!(found_tos.len(), 3);
        // check if the textual objects are found
        assert_eq!(&found_tos[0].ticket_id, &sample_to1.ticket_id);
    }

    // test get unique ticket id
    #[tokio::test]
    async fn test_get_unique_ticket_id() {
        let random_database_dir = get_random_test_database_dir();
        // create a new TextualObjectMachineRs with SQLITE store
        let mut tom = ToMachine::new(&random_database_dir, StoreType::SQLITE, Some(ToMachineOption{
            use_random_file_name: true,
            ..Default::default()
        })).await;
        // check if the machine is created
        let unique_ticket_id = tom.get_unique_ticket_id().await;
        // check if the ticket id is unique
        assert_eq!(unique_ticket_id.len(), 5);
    }
}
