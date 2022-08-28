use crate::to_card::to_card_struct::ToCardField;
use strum::IntoEnumIterator;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToCardConvertRule {
    // The field in `Card` to map the source field to.
    pub card_field: String,
    // The fields from which to get the value to map to the card field.
    // If the first field specified has no value or does not exist in the object, the next field will be used.
    // If none of the fields has value, the card field will be set to empty.
    pub source_fields: Vec<String>,
}

impl ToCardConvertRule {
    pub fn is_card_field_valid(&self) -> bool {
        // iterate over toCardField enum
        for field in ToCardField::iter() {
            if field.to_string() == self.card_field {
                return true;
            }
        }
        false
    }

}

// test mod
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_tell_if_card_field_is_valid() {
        let invalid_rule = ToCardConvertRule {
            card_field: "card_field".to_string(),
            source_fields: vec!["source_field".to_string()],
        };
        assert!(!invalid_rule.is_card_field_valid());

        let valid_rule = ToCardConvertRule {
            card_field: ToCardField::Id.to_string(),
            source_fields: vec!["source_field".to_string()],
        };
        assert!(valid_rule.is_card_field_valid());
    }
}