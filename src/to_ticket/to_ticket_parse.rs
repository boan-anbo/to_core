use chrono::{DateTime, FixedOffset, NaiveDateTime};

use crate::to_parser::parser_option::ToParserOption;
use crate::to_ticket::to_ticket_position::ToTicketPositionInfo;
use crate::to_ticket::to_ticket_struct::ToTicket;

impl ToTicket {
    /// Parse single ticket from text
    /// # Arguments
    /// * `marked_content_only` - raw text to parse
    pub fn parse(marked_content_only: &str, opt: &ToParserOption, intext_position: Option<ToTicketPositionInfo>) -> Self {

        // remove left and right markers if they exist
        let mut clean_content = marked_content_only.replace(&opt.to_marker.left_marker, "");
        clean_content = clean_content.replace(&opt.to_marker.right_marker, "");


        // split strings with separator for once
        let split_content = clean_content.split(&opt.to_marker.value_entry_separator);
        // iterate through the split strings
        let mut to_ticket = ToTicket::default();
        // load position if exists
        to_ticket.to_intext_option = intext_position;
        for content in split_content {
            // split the string with separator for once, ignoring the value separator if it comes after the key separator
            let mut split_content = content.splitn(2, &opt.to_marker.value_separator);
            // get the key and value
            let mut key = split_content.next().unwrap_or("").to_string();
            // if key is empty, continue
            // make sure the value is not empty, if it is, set it to empty string
            let mut value = split_content.next().unwrap_or("").to_string();

            // this occurs when e.g. [[:value_with_no_key]]
            if key.is_empty() {
                // check if value is empty, if it is, continue
                if value.is_empty() {
                    continue;
                } else {
                    // if value is not empty, set the value to the key and set value to empty string
                    key = value;
                    value = "".to_string();
                }
            }

            // check if the key is a reserved field, if so, assign it to the corresponding field
            match key.as_ref() {
                "id" => { to_ticket.ticket_id = value }
                "updated" => {
                    let naive_updated = NaiveDateTime::parse_from_str(&value, &opt.date_format).unwrap();
                    to_ticket.to_updated = DateTime::<FixedOffset>::from_utc(naive_updated, FixedOffset::east(0));
                }
                "store_id" => {
                    to_ticket.to_store_url = Some(value);
                }
                "store_info" => {
                    to_ticket.to_store_info = Some(value);
                }
                _ => {
                    // if the key is not a reserved field, add it to the values
                    to_ticket.values.insert(key.to_string(), value);
                }
            }
        }
        to_ticket
    }


    fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }
}


// test create default TextualObjectTicket
#[cfg(test)]
mod tests {
    /*
Parser tests
 */
    use chrono::{Datelike, TimeZone, Utc};

    use crate::to_parser::parser_option::ToParserOption;
    use crate::to_ticket::to_ticket_struct::ToTicket;

    #[test]
    fn test_parse() {
        let mark_content = "key1:value1|key2:value2";
        let p1 = ToParserOption::default();
        let to_ticket = ToTicket::parse(mark_content, &p1, None);
        assert_eq!(to_ticket.values.len(), 2);
        assert_eq!(to_ticket.values.get("key1").unwrap(), "value1");
        assert_eq!(to_ticket.values.get("key2").unwrap(), "value2");
    }

    #[test]
    fn test_parse_string_with_markers() {
        let opt = ToParserOption::default();
        let mark_content = format!("{}key1:value1|key2:value2{}", opt.to_marker.left_marker, opt.to_marker.right_marker);
        let to_ticket = ToTicket::parse(&mark_content, &opt, None);
        assert_eq!(to_ticket.values.len(), 2);
        assert_eq!(to_ticket.values.get("key1").unwrap(), "value1");
        assert_eq!(to_ticket.values.get("key2").unwrap(), "value2");
    }


    // test parse with meta-data
    #[test]
    fn test_parse_with_meta_data() {
        let opt = ToParserOption::default();
        let mark_content = format!("{}id:test_id|key1:value1|key2:value2|updated:2018-01-01 00:00:00|store_info:store_info|store_id:store_id{}", opt.to_marker.left_marker, opt.to_marker.right_marker);
        let to_ticket = ToTicket::parse(&mark_content, &opt, None);
        assert_eq!(to_ticket.ticket_id, "test_id".to_string());
        assert_eq!(to_ticket.values.len(), 2);
        assert_eq!(to_ticket.values.get("key1").unwrap(), "value1");
        assert_eq!(to_ticket.values.get("key2").unwrap(), "value2");
        assert_eq!(to_ticket.to_updated.num_days_from_ce(), Utc.ymd(2018, 1, 1).num_days_from_ce());
        assert_eq!(to_ticket.to_store_url, Some("store_id".to_string()));
        assert_eq!(to_ticket.to_store_info, Some("store_info".to_string()));
    }

    #[test]
    fn test_parse_with_missing_values() {
        let opt = ToParserOption::default();
        let mark_content = format!("{}id:test_id|key1:|key2|:value1|updated:2018-01-01 00:00:00|store_info:store_info{}", opt.to_marker.left_marker, opt.to_marker.right_marker);
        let to_ticket = ToTicket::parse(&mark_content, &opt, None);
        assert_eq!(to_ticket.ticket_id, "test_id".to_string());
        assert_eq!(to_ticket.values.len(), 3);
        assert_eq!(to_ticket.values.get("key1").unwrap(), "");
        assert_eq!(to_ticket.values.get("key2").unwrap(), "");
        assert_eq!(to_ticket.to_updated.num_days_from_ce(), Utc.ymd(2018, 1, 1).num_days_from_ce());
        assert_eq!(to_ticket.to_store_url, None);
        assert_eq!(to_ticket.to_store_info, Some("store_info".to_string()));
    }

    // test reading from json
    #[test]
    fn test_from_json() {
        let json = r#"{
            "id": "test_id",
            "ticket_id": "12345",
            "to_updated": "2018-01-01T19:20:30.45+01:00",
            "to_store_url": "store_url_value",
            "to_store_info": "store_info_value",
            "values": [
                ["key1", "value1"],
                ["key2", "value2"]
            ]
        }"#;
        let to_ticket = ToTicket::from_json(json);
        assert_eq!(to_ticket.ticket_id, "12345".to_string());
        assert_eq!(to_ticket.values.len(), 2);
        assert_eq!(to_ticket.values.get("key1").unwrap(), "value1");
        assert_eq!(to_ticket.values.get("key2").unwrap(), "value2");
        assert_eq!(to_ticket.to_updated.num_days_from_ce(), Utc.ymd(2018, 1, 1).num_days_from_ce());
        assert_eq!(to_ticket.to_store_url, Some("store_url_value".to_string()));
        assert_eq!(to_ticket.to_store_info, Some("store_info_value".to_string()));
    }
}