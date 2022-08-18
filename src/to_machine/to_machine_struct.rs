/*
The main entry point of TO application. This needs to be written in as many languages as there is a need for.
This is the Rust version.
 */
use std::path::Path;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use crate::enums::store_type::StoreType;
use wasm_bindgen::prelude::*;
use crate::db::db_op::{connect_to_database, initialize_database, join_db_path};
use crate::db::to_db_op::{count_textual_objects, insert_to};
use crate::to::textual_object::TextualObject;
use crate::to_machine::to_machine_option::ToMachineOption;

#[derive(Debug, PartialEq, Clone)]
pub struct TextualObjectMachine {
    // store type
    pub(crate) store_type: StoreType,
    // store path, that implements Copy
    pub(crate) store_path: String,
    // number of tos in the store, read only for the outside world
    pub(crate) to_count: i64,

}

// default constructor for TextualObjectMachineRs
impl TextualObjectMachine {
    pub async fn new(store_directory: &str, store_type: StoreType, input_opt: Option<ToMachineOption>) -> Self {
        let mut to_count = 0;


        // check if the opt.store_file_name is specified, defaults to _to_store.db
        let mut store_file_name = "_to_store.db".to_string();
        if let Some(opt) = input_opt {
            if let Some(store_file_name_opt) = opt.store_file_name {
                store_file_name = store_file_name_opt.to_string();
            }
        }

        match store_type {
            StoreType::JSON => {
                // create a new TextualObjectMachineRs with JSON store
                // check if json file exists, if not, throw an error
                // if !Path::new(&store_path_str).exists() {
                //     panic!("JSON file does not exist");
                // }
            }
            StoreType::SQLITE => {
                // create a new TextualObjectMachineRs with SQLITE store
                // check if sqlite file exists, if not, throw an error
                let re = initialize_database(store_directory, &store_file_name).await;
                if re.is_err() {
                    panic!("Check file conflict: cannot initialize database at {}", join_db_path(store_directory, &store_file_name));
                }
                // get pool
                let pool = connect_to_database(store_directory).await;
                // get count of tos in the store
                to_count = count_textual_objects(&pool).await;
                match re {
                    Ok(_) => {}
                    // print out error message
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            }
        }

        TextualObjectMachine {
            store_type,
            store_path: String::from(join_db_path(store_directory, &store_file_name)),
            to_count,
        }
    }
}

// implementing getters and setters for TextualObjectMachineRs
impl TextualObjectMachine {
    pub fn get_store_type(&self) -> StoreType {
        self.store_type
    }
    pub fn get_store_path(&self) -> String {
        self.store_path.clone()
    }
    pub fn get_to_count(&self) -> i64 {
        self.to_count
    }
    pub fn set_to_count(&mut self, to_count: i64) {
        self.to_count = to_count;
    }
}

// implement uitility functions for TextualObjectMachine
impl TextualObjectMachine {
    pub(crate) async fn get_pool(&self) -> Pool<Sqlite> {
        connect_to_database(&self.store_path).await
    }
}


// tests for TextualObjectMachineRs
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use uuid::Uuid;
    use crate::enums::store_type::StoreType;
    use crate::to::textual_object::TextualObject;
    use crate::to_machine::to_machine_option::ToMachineOption;
    use crate::to_machine::to_machine_struct::TextualObjectMachine;
    use crate::utils::id_generator::generate_id;


    // initiate for tests

    pub fn get_test_asset_path(file_name: Option<&str>) -> String {
        let mut cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cargo_dir.push("resources/test/");
        if let Some(file_name) = file_name {
            cargo_dir.push(file_name);
        } else {}
        cargo_dir.into_os_string().into_string().unwrap()
    }


    // test new() db with existent sqlite
    #[tokio::test]
    async fn test_initialize_tom_with_random_store_name() {
        let test_db_file_name = generate_id();
        let existent_sqlite_file = get_test_asset_path(Some(test_db_file_name.as_str()));
        // create a new TextualObjectMachineRs with SQLITE store
        let machine = TextualObjectMachine::
        new(&existent_sqlite_file, StoreType::SQLITE,
            Some(
                ToMachineOption::new().set_store_file_name(
                    Some(test_db_file_name.as_str())
                )
            )).await;
        // check if the machine is created
        assert_eq!(machine.store_type, StoreType::SQLITE);
        assert_eq!(machine.to_count, 0);
    }
}