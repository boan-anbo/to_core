extern crate core;

pub mod entities;
pub(crate) mod utils;
pub mod to_machine;
pub mod to_ticket;
pub mod enums;
pub mod to;
pub mod db;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
