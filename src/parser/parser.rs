use regex::{Captures, escape, Match, Regex};
use crate::entities::to_ticket::{TextualObjectTicket};
use crate::entities::to_ticket_position::ToTicketInTextPosition;
use crate::parser::parser_option::ParserOption;

// create lazy static regex for parsing
// use regular expressions to match all slices of text that starts with left marker and ends with right marker
pub fn match_all_marks(text: &str, opt: ParserOption) -> Vec<TextualObjectTicket> {
    let lines: &Vec<String> = &text.lines().map(|s| s.to_string()).collect();
    let re = Regex::new(format!(r"{}(.*?){}", escape(&opt.to_marker.left_marker), escape(&opt.to_marker.right_marker)).as_str()).unwrap();
    let mut result = Vec::new();
    // iterate with line number
    for (line_number, line) in lines.iter().enumerate() {
        // iterate with match
        for m in re.captures_iter(line) {
            // get the match position
            let position = ToTicketInTextPosition::from_match(&m, line_number);
            // get first group of match
            let content = m.get(1).unwrap().as_str();
            // parse the match
            let to_ticket = TextualObjectTicket::parse(content, &opt, Some(position));
            // add the match to the result
            result.push(to_ticket);
        }
    }

    result
}

// test create default TextualObjectTicket
#[cfg(test)]
mod tests {
    use crate::parser::parser::match_all_marks;
    use crate::parser::parser_option::ParserOption;

    #[test]
    fn test_one_mark() {
        let raw_text = "[[id:1]]";
        let opt = ParserOption::default();
        let result = match_all_marks(raw_text, opt);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "1");
    }

    // test two marks
    #[test]
    fn test_two_marks() {
        let raw_text = "[[id:1]][[id:2]]";
        let opt = ParserOption::default();
        let result = match_all_marks(raw_text, opt);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, "1");
        assert_eq!(result[1].id, "2");
    }

    // test two marks with different positions
    #[test]
    fn test_two_marks_different_positions() {
        let raw_text = "[[id:1]]\n[[id:2]]";
        let opt = ParserOption::default();
        let result = match_all_marks(raw_text, opt);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, "1");
        assert_eq!(result[1].id, "2");
    }

    // test three marks with different positions
    #[test]
    fn test_three_marks_different_positions() {
        let raw_text = "[[id:1]]\n[[id:2]]\n[[id:3]]";
        let opt = ParserOption::default();
        let result = match_all_marks(raw_text, opt);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, "1");
        assert_eq!(result[1].id, "2");
        assert_eq!(result[2].id, "3");
    }

    // test one mark position
    #[test]
    fn test_one_mark_position() {
        let indent = "12345";
        let text = "[[id:1]]";
        let raw_text = format!("{}{}", indent, text);
        let opt = ParserOption::default();
        let result = match_all_marks(&raw_text, opt);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "1");
        assert_eq!(result[0].to_intext_option.as_ref().unwrap().line, 0);
        assert_eq!(result[0].to_intext_option.as_ref().unwrap().column, indent.len());
        assert_eq!(result[0].to_intext_option.as_ref().unwrap().length, text.len());
    }

    // test one mark position in second line
    #[test]
    fn test_one_mark_position_in_second_line() {
        let indent = "12345";
        let text = "[[id:1]]";
        let raw_text = format!("\n{}{}", indent, text);
        let opt = ParserOption::default();
        let result = match_all_marks(&raw_text, opt);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "1");
        assert_eq!(result[0].to_intext_option.as_ref().unwrap().line, 1);
        assert_eq!(result[0].to_intext_option.as_ref().unwrap().column, indent.len());
        assert_eq!(result[0].to_intext_option.as_ref().unwrap().length, text.len());
    }


}
