use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, PartialEq, Clone, Copy)]
#[wasm_bindgen]
pub enum StoreType {
    // default to JSON store
    JSON,
    // the store is a file
    SQLITE,
}