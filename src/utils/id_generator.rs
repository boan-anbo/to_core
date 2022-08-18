use nanoid::nanoid;

pub fn generate_id() -> String {
    nanoid!(5)
}

// write tests for this module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let id = generate_id();
        assert_eq!(id.len(), 5);
    }
}
