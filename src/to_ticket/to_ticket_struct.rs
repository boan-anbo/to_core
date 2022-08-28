use chrono::{DateTime, FixedOffset, Local};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};


use crate::to_ticket::to_ticket_marker::ToMarker;
use crate::to_ticket::to_ticket_position::ToTicketPositionInfo;
use crate::utils::id_generator::generate_id;

#[derive(Serialize, Deserialize, Clone)]
pub struct ToTicket {
    // unique ID in the local storage
    pub id: String,
    // unique ticket Id in the local storage
    pub ticket_id: String,
    // values: indexMap of keys and values; uses indexMap rather than HashMap because IndexMap preserves the insertion orderj
    #[serde(with = "indexmap::serde_seq")]
    pub values: IndexMap<String, String>,
    /*
    Public meta-data: the fields below are reserved for meta-data.
    In output, they are written with a PREFIX;
     */
    // updated date field: Chrono::DateTime
    #[serde(default)]
    pub to_updated: DateTime<FixedOffset>,
    // redable notes on storage location of the referenced TO
    #[serde(default)]
    pub to_store_url: Option<String>,
    // Optional unique ID of the storage field
    #[serde(default)]
    pub to_store_info: Option<String>,

    /*
    Private meta-data, not be printed
     */
    #[serde(default)]
    pub to_marker: ToMarker,
    #[serde(default)]
    pub to_intext_option: Option<ToTicketPositionInfo>,

}

// create default TextualObjectTicket
impl Default for ToTicket {
    fn default() -> Self {
        ToTicket {
            id: String::new(),
            ticket_id: generate_id(),
            values: IndexMap::new(),
            to_updated: Local::now().with_timezone(&FixedOffset::east(0)),
            to_store_url: None,
            to_store_info: None,
            to_marker: ToMarker::default(),
            to_intext_option: None,
        }
    }

}




// test create default TextualObjectTicket
#[cfg(test)]
mod tests {
    use chrono::{Datelike, Utc};

    use super::*;

    #[test]
    fn test_create_ticket() {
        let ticket = ToTicket::default();
        assert_eq!(ticket.ticket_id.len(), 5);
        assert_eq!(ticket.values.len(), 0);
        assert_eq!(ticket.to_updated.num_days_from_ce(), Utc::now().num_days_from_ce());
        assert_eq!(ticket.to_store_url, None);
        assert_eq!(ticket.to_store_info, None);
        assert!(ticket.to_marker.left_marker.len() > 0);
    }




}