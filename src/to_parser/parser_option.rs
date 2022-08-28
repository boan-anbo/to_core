use crate::to_ticket::to_ticket_marker::ToMarker;

pub struct ToParserOption {
    pub to_marker: ToMarker,
    pub date_format: String,
}

impl Default for ToParserOption {
    fn default() -> Self {
        ToParserOption {
            to_marker: ToMarker::default(),
            date_format: String::from("%Y-%m-%d %H:%M:%S"),
        }
    }
}