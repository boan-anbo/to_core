use regex::{Captures, Match, Regex};
use crate::entities::to_ticket::{TextualObjectTicket};
use crate::parser::parser_option::ParserOption;

// parse all the markers in the text, and return a vector of to tickets
pub fn parse(text: &str, opt: Option<ParserOption>) -> Vec<TextualObjectTicket> {
    vec![]
}


//

// use regular expressions to match all slices of text that starts with left marker and ends with right marker
pub fn match_all_marks(text: &str, opt: ParserOption) -> Vec<TextualObjectTicket> {
    let regex = Regex::new(&format!("{}(.*){}", opt.to_marker.left_marker, opt.to_marker.right_marker)).unwrap();
    let all_matches = regex.captures_iter(text);
    let mut result = Vec::new();
    // use only group 1
    for m in all_matches.iter {
        let text = m.get(0).unwrap();

        let parsed_to = TextualObjectTicket::parse(text.as_str(), &ParserOption::default());
        result.push(parsed_to);
    }
    result
}

// test create default TextualObjectTicket
#[cfg(test)]
mod tests {
    use crate::parser::parser::match_all_marks;
    use crate::parser::parser_option::ParserOption;

    #[test]
    fn test() {
        let raw_text = "[[id:1]]";
        let opt = ParserOption::default();
        let result = match_all_marks(raw_text, opt);
        assert_eq!(result.len(), 1)
    }
}
