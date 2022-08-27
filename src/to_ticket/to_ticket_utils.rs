use crate::to_ticket::to_ticket_marker::ToMarker;


pub fn print_minimal_ticket(ticket_id: &str, opt: Option<ToMarker>) -> String {
    // if opt is None, use default
    let to_marker = opt.unwrap_or(ToMarker::default());

    // create a list of string to be added
    let mut print_label: Vec<String> = Vec::new();
    // add id
    print_label.push(format!("id: {}", ticket_id));
    // join all the strings in the list with the a separator |
    // and join with the to_marker.left_marker and to_marker.right_marker
    let mut result = String::new();
    result.push_str(&to_marker.left_marker);
    result.push_str(&print_label.join(&format!(" {} ", to_marker.value_entry_separator)));
    result.push_str(&to_marker.right_marker);

    result
}

// test print_minimal_ticket
#[test]
fn test_print_minimal_ticket() {
    let ticket_id = "test_id";
    let print_label = print_minimal_ticket(ticket_id, None);
    assert_eq!(print_label, format!("[[id: test_id]]"));

}