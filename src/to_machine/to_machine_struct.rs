/*
The main entry point of TO application. This needs to be written in as many languages as there is a need for.
This is the Rust version.
 */
use std::path::Path;
use crate::enums::store_type::StoreType;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, PartialEq, Clone)]
#[wasm_bindgen]
pub struct TextualObjectMachineRs {
    // store type
    store_type: StoreType,
    // store path, that implements Copy
    store_path: String,

}

// default constructor for TextualObjectMachineRs
#[wasm_bindgen]
impl TextualObjectMachineRs {
    #[wasm_bindgen(constructor)]
    pub fn new(store_path: &str, store_type: StoreType) -> Self {
        // match store_type
        match store_type {
            StoreType::JSON => {
                // create a new TextualObjectMachineRs with JSON store
                // check if json file exists, if not, throw an error
                if !Path::new(store_path).exists() {
                    panic!("JSON file does not exist");
                }

            }
            StoreType::SQLITE => {
                // throw unimplmented error
                unimplemented!();
            }
        }

        TextualObjectMachineRs {
            store_type,
            store_path: String::from(store_path),
        }
    }
}


// tests for TextualObjectMachineRs
#[cfg(test)]
mod tests {
    
    use std::path::PathBuf;
    use crate::enums::store_type::StoreType;
    use crate::to_machine::to_machine_struct::TextualObjectMachineRs;


    // initiate for tests

    fn get_test_asset_path(file_name: &str) -> String {
        let mut cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cargo_dir.push("resources/test/");
        cargo_dir.push(file_name);
        // convert the PathBuf to path string
        cargo_dir.into_os_string().into_string().unwrap()

    }
    // test new() with non_existent_json
    #[test]
    #[should_panic]
    fn test_new() {

        let non_existent_json_file = get_test_asset_path("non_existent_json_file.json");
        // create a new TextualObjectMachineRs with JSON store
        let _machine = TextualObjectMachineRs::new(&non_existent_json_file, StoreType::JSON);
    }

    // test new() with existent json
    #[test]
    fn test_new_existent_json() {
        let existent_json_file = get_test_asset_path("existent_json_file.json");
        // create a new TextualObjectMachineRs with JSON store
        let machine = TextualObjectMachineRs::new(&existent_json_file, StoreType::JSON);
        // check if the machine is created
        assert_eq!(machine.store_type, StoreType::JSON);
        assert_eq!(machine.store_path, existent_json_file);
    }




}