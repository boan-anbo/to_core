use crate::to_ticket::to_ticket_marker::ToMarker;

pub struct ParserOption {
    pub to_marker: ToMarker,
    pub date_format: String,
}

impl Default for ParserOption {
    fn default() -> Self {
        ParserOption {
            to_marker: ToMarker::default(),
            date_format: String::from("%Y-%m-%d %H:%M:%S"),
        }
    }
}