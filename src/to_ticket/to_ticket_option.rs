
pub struct ToTicketPrintOption {
    // whether include updated
    pub include_updated: bool,
    // whether include created
    pub include_created: bool,
    // whether include store_info
    pub include_store_info: bool,
    // whether include store_id
    pub include_store_id: bool,
    // this will overwrite other options and print only a ticket with a single ticket id field
    pub minimal: bool,
}

// create default values for ToTicketPrintOption

impl Default for ToTicketPrintOption {
    fn default() -> Self {
        ToTicketPrintOption {
            include_updated: true,
            include_created: true,
            include_store_info: true,
            include_store_id: true,
            minimal: false,
        }
    }
}