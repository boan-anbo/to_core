use nanoid::nanoid;

pub mod id_generator;

pub fn generate_id() -> String {
    nanoid!(7)
}

// write tests for this module
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_id() {
        let id = generate_id();
        assert_eq!(id.len(), 7);

    }
}