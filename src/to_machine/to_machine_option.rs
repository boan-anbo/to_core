use crate::enums::store_type::StoreType;

#[derive(Debug, PartialEq, Clone)]
pub struct ToMachineOption {
    // use random file name
    pub use_random_file_name: bool,
    // specify a name for the store, rather than the default name: _to_store.db
    pub store_file_name: Option<String>,

    // specify store information, describing what the store does
    pub store_info: Option<String>,

    // store type
    pub store_type: StoreType,
}

// impl default for ToMachineOption
impl Default for ToMachineOption {
    fn default() -> Self {
        ToMachineOption {
            use_random_file_name: false,
            store_file_name: None,
            store_info: Some("A TO Store".to_string()),
            store_type: StoreType::SQLITE
        }
    }
}

impl ToMachineOption {
    pub(crate) fn set_store_file_name(&self, p0: Option<&str>) -> ToMachineOption {
        ToMachineOption {
            store_file_name: p0.map(|p0| p0.to_string()),
            ..self.clone()
        }
    }
}

// implement default new function for ToMachineOption with default store_type: SQLITE
impl ToMachineOption {
    pub fn new() -> Self {
        ToMachineOption::default()
    }
}