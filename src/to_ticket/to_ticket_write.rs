use crate::to_ticket::to_ticket_option::ToTicketPrintOption;
use crate::to_ticket::to_ticket_struct::ToTicket;

// writer methods
impl ToTicket {
    pub fn print(&self, opt: Option<ToTicketPrintOption>) -> String {
        // if not optioin, use default option
        let opt = opt.unwrap_or_default();
        // create a list of string to be added
        let mut print_label: Vec<String> = Vec::new();
        // add id
        print_label.push(format!("id: {}", self.ticket_id));

        if !opt.minimal {
            // add values; use reverse order otherwise the first inserted is the last printed
            for (key, value) in self.values.iter() {
                // ignore if the key is reserved, i.e. the same as the public or private meta-data fields
                if key == "to_updated" || key == "to_store_id" || key == "to_store_info" || key == "to_marker" {
                    continue;
                }
                print_label.push(format!("{}: {}", key, value));
            }

            /*
        Add meta-data if needed and at the end of the ticket
         */
            // check opt to see if include_updated
            // if so, print date string without nano-second
            if opt.include_updated {
                print_label.push(format!("updated: {}", self.to_updated.format("%Y-%m-%d %H:%M:%S")));
            }
            // check opt to see if include_store_info, length of store_id is not None, and length of store_id is not 0
            if opt.include_store_info && self.to_store_url.is_some() && self.to_store_url.as_ref().unwrap().len() > 0 {
                print_label.push(format!("store_info: {}", self.to_store_url.clone().unwrap()));
            }
            // check opt to see if include_store_id, length of store_id is not None, and length of store_id is not 0
            if opt.include_store_id && self.to_store_info.is_some() && self.to_store_info.as_ref().unwrap().len() > 0 {
                print_label.push(format!("store_id: {}", self.to_store_info.clone().unwrap()));
            }
        }
        // join all the strings in the list with the a separator |
        // and join with the to_marker.left_marker and to_marker.right_marker
        let mut result = String::new();
        result.push_str(&self.to_marker.left_marker);
        result.push_str(&print_label.join(&format!(" {} ", self.to_marker.value_entry_separator)));
        result.push_str(&self.to_marker.right_marker);

        result
    }
    /*
    Print with least information possible
    */
    pub fn print_minimal(&self) -> String {
        self.print(Some(ToTicketPrintOption {
            include_updated: false,
            include_store_info: false,
            include_store_id: false,
            include_created: false,
            minimal: true,
        }))
    }

    // json writer
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

// test module
#[cfg(test)]
mod tests {
    use chrono::{FixedOffset, TimeZone, Utc};

    use crate::to::to_struct::TextualObject;
    use crate::to_ticket::to_ticket_option::ToTicketPrintOption;
    use crate::to_ticket::to_ticket_struct::ToTicket;

    #[test]
    fn test_print_ticket_empty() {
        let mut ticket = ToTicket::default();
        ticket.ticket_id = "test_id".to_string();
        let print_label = ticket.print(None);
        assert_eq!(print_label, format!("[[id: test_id | updated: {}]]", Utc::now().format("%Y-%m-%d %H:%M:%S")));
    }

    // test print ticket with values
    #[test]
    fn test_print_ticket_with_values() {
        let mut ticket = ToTicket::default();
        ticket.ticket_id = "test_id".to_string();
        ticket.values.insert("key1".to_string(), "value1".to_string());
        ticket.values.insert("key2".to_string(), "value2".to_string());
        let print_label = ticket.print(None);
        assert_eq!(print_label, format!("[[id: test_id | key1: value1 | key2: value2 | updated: {}]]", Utc::now().format("%Y-%m-%d %H:%M:%S")));
    }

    // test print ticket with values and meta-data
    #[test]
    fn test_print_ticket_with_values_and_meta_data() {
        let mut ticket = ToTicket::default();
        ticket.ticket_id = "test_id".to_string();
        ticket.values.insert("key1".to_string(), "value1".to_string());
        ticket.values.insert("key2".to_string(), "value2".to_string());
        ticket.to_updated = FixedOffset::east(0).ymd(2019, 1, 1).and_hms(0, 0, 0);
        ticket.to_store_url = Some("store_info_value".to_string());
        ticket.to_store_info = Some("store_id_value".to_string());
        let print_label = ticket.print(None);
        assert_eq!(print_label, format!("[[id: test_id | key1: value1 | key2: value2 | updated: 2019-01-01 00:00:00 | store_info: store_info_value | store_id: store_id_value]]"));
    }

    // test when print ticket values has keys that conflict with meta-data
    #[test]
    fn test_print_ticket_with_values_and_meta_data_conflict() {
        let mut ticket = ToTicket::default();
        ticket.ticket_id = "test_id".to_string();
        ticket.values.insert("key1".to_string(), "value1".to_string());
        ticket.values.insert("key2".to_string(), "value2".to_string());
        ticket.to_updated = FixedOffset::east(0).ymd(2019, 1, 1).and_hms(0, 0, 0);
        ticket.to_store_url = Some("correct_store_info_value".to_string());
        ticket.to_store_info = Some("correct_store_info_value".to_string());
        ticket.values.insert("to_store_id".to_string(), "wrong_store_id_value".to_string());
        ticket.values.insert("to_store_info".to_string(), "wrong_store_info_value".to_string());
        let print_label = ticket.print(None);
        assert_eq!(print_label, format!("[[id: test_id | key1: value1 | key2: value2 | updated: 2019-01-01 00:00:00 | store_info: correct_store_info_value | store_id: correct_store_info_value]]"));
    }

    // test print minimal ticket with only id field
    #[test]
    fn test_print_minimal_ticket_with_only_id() {
        let to = TextualObject::get_sample();
        let ticket_id = to.ticket_id.clone();
        let ticket = ToTicket::from(to);
        let minimal_label = ticket.print(Some(ToTicketPrintOption {
            include_updated: true,
            include_store_info: true,
            include_store_id: true,
            include_created: true,
            minimal: true, // i set other options to true but those should be overridden when minimal is true
        }));
        assert_eq!(minimal_label, format!("[[id: {}]]", ticket_id));
        let _not_minimal_label = ticket.print(Some(ToTicketPrintOption {
            include_updated: true,
            include_store_info: true,
            include_store_id: true,
            include_created: true,
            minimal: false,
        }));
    }
}