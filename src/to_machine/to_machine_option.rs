
#[derive(Debug, PartialEq, Clone)]
pub struct ToMachineOption {
    // specify a name for the store, rather than the default name: _to_store.db
    pub store_file_name: Option<String>,

    // store type
    pub store_type: String,
}

impl ToMachineOption {
    pub(crate) fn set_store_file_name(&self, p0: Option<&str>) -> ToMachineOption {
        ToMachineOption {
            store_file_name: p0.map(|p0| p0.to_string()),
            ..self.clone()
        }
    }
}

// implement default new function for ToMachineOption
impl ToMachineOption {
    pub fn new() -> Self {
        ToMachineOption {
            store_file_name: None,
            store_type: "SQLITE".to_string(),
        }
    }
}