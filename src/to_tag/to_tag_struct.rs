

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use crate::to_parser::parser::ToParser;


use crate::to_parser::parser_option::ToParserOption;
use crate::to_ticket::to_ticket_marker::ToMarker;
use crate::to_ticket::to_ticket_struct::ToTicket;

#[derive(Deserialize, Serialize, Clone)]
pub struct ToTag {
    key: String,
    value: Option<String>,
    note: Option<String>,
}

// implement from TextualObjectTicket

impl From<ToTicket> for ToTag {
    fn from(to_ticket: ToTicket) -> ToTag {
        let values: IndexMap<String, String> = to_ticket.values;
        let mut key = String::new();
        if values.len() >= 1 {
            key = values.keys().nth(0).unwrap().to_string();
        }
        let mut value: Option<String> = None;
        if values.len() >= 2 {
            let second_key = values.keys().nth(1).unwrap().to_string();
            value = Some(second_key);
        }
        let mut note: Option<String> = None;
        if values.len() >= 3 {
            let third_key = values.keys().nth(2).unwrap().to_string();
            note = Some(third_key);

            // if there are more keys, add all to note to note and separate with comma
            for key in values.keys().skip(3) {
                note = Some(format!("{}, {}", note.unwrap(), key));
            }
        }

        ToTag {
            key,
            value,
            note,
        }
    }
}


impl ToTag {

    // return (cleaned text, list of tags).
    // Cleaned text = text without tags.
    // List of tags = list of tags found in text.
    pub fn scan_text_for_tags(text: &str) -> (String, Vec<ToTag>) {
        let all_tickets = ToParser::scan_text_for_tickets(text, ToParserOption::default());
        let mut tags: Vec<ToTag> = Vec::new();
        for ticket in all_tickets {
            let tag = ToTag::from(ticket);
            tags.push(tag);
        }
        // replace all tags with empty string
        let mut text = text.to_string();
        for tag in &tags {
            text = text.replace(&tag.print_tag(None), "");
        }
        (text, tags)


    }

    // implement print tag
    pub fn print_tag(&self, to_mark: Option<ToMarker>) -> String {
        // create default to_marker if not provided
        let mut to_marker = ToMarker::default();
        if to_mark.is_some() {
            to_marker = to_mark.unwrap();
        }
        let mut tag = vec![self.key.clone()];
        if self.value.is_some() {
            tag.push(self.value.clone().unwrap());
        }
        if self.note.is_some() {
            tag.push(self.note.clone().unwrap());
        };
        // surround content by to_marker.start and to_marker.end, and separate with to_marker.separator
        let tag = tag.join(&to_marker.value_entry_separator);
        format!("{}{}{}", to_marker.left_marker, tag, to_marker.right_marker)

        // add value if it exists with separator
    }
}

// tests
#[cfg(test)]
mod test {
    use std::borrow::Borrow;

    

    use crate::to_tag::to_tag_struct::ToTag;
    use crate::to_parser::parser_option::ToParserOption;
    use crate::to_parser::parser::ToParser;

    // test from ticket to tag
    #[test]
    fn test_from_ticket_to_tag() {
        let raw_text = "[[KEY|VALUE|NOTE]]";
        let result = ToParser::scan_text_for_tickets(raw_text, ToParserOption::default());
        let first_ticket = result[0].clone();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].values.get("KEY").unwrap(), "");
        let tag = ToTag::from(first_ticket);
        assert_eq!(&tag.key, "KEY");
        assert_eq!(&tag.value.unwrap(), "VALUE");
        assert_eq!(&tag.note.unwrap(), "NOTE");
    }

    // test noise ticket to tag
    #[test]
    fn test_noise_ticket_to_tag() {
        let raw_text = "[[KEY:value|VALUE:2|:3]]";
        let result = ToParser::scan_text_for_tickets(raw_text, ToParserOption::default());
        let first_ticket = result[0].clone();
        assert_eq!(result.len(), 1);
        let tag = ToTag::from(first_ticket);
        assert_eq!(&tag.key, "KEY");
        assert_eq!(&tag.value.as_ref().unwrap().clone(), "VALUE");
        assert_eq!(&tag.note.as_ref().unwrap().clone(), "3");
        let tag_print = tag.clone().borrow().print_tag(None);
        assert_eq!(tag_print, "[[KEY|VALUE|3]]");
    }

    // test ticket to tag, key and value only
    #[test]
    fn test_ticket_to_tag_key_value_only() {
        let raw_text = "[[KEY:value|VALUE:2]]";
        let result = ToParser::scan_text_for_tickets(raw_text, ToParserOption::default());
        let first_ticket = result[0].clone();
        assert_eq!(result.len(), 1);
        let tag = ToTag::from(first_ticket);
        assert_eq!(&tag.key, "KEY");
        assert_eq!(&tag.value.as_ref().unwrap().clone(), "VALUE");
        let tag_print = tag.clone().borrow().print_tag(None);
        assert_eq!(tag_print, "[[KEY|VALUE]]");
    }

    // test ticket to tag, key only
    #[test]
    fn test_ticket_to_tag_key_only() {
        let raw_text = "[[KEY:value]]";
        let result = ToParser::scan_text_for_tickets(raw_text, ToParserOption::default());
        let first_ticket = result[0].clone();
        assert_eq!(result.len(), 1);
        let tag = ToTag::from(first_ticket);
        assert_eq!(&tag.key, "KEY");
        let tag_print = tag.clone().borrow().print_tag(None);
        assert_eq!(tag_print, "[[KEY]]");
    }

    // test scan text for tags
    #[test]
    fn test_scan_text_for_tags() {
        let raw_text = "1[[KEY|VALUE|NOTE]]\n2[[KEY2|VALUE2|NOTE2]]\n3[[KEY3|VALUE3|NOTE3]]";
        let _result = ToParser::scan_text_for_tickets(raw_text, ToParserOption::default());
        let (cleaned, tags) = ToTag::scan_text_for_tags(&raw_text);
        assert_eq!(cleaned, "1\n2\n3");
        assert_eq!(tags.len(), 3);
    }
}