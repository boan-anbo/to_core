use crate::db::db_op::reset_database;
use crate::to_machine::to_machine_struct::ToMachine;

pub mod to_machine_struct;
pub mod to_machine_option;
pub mod to_machine_op;
pub mod to_machine_db;
pub mod to_machine_pub_op;

// implement db related methods for TextualObjectMachine
impl ToMachine {
    // clear all tables;
    pub async fn reset_db(&self) -> () {
        reset_database(self.store_url.as_ref()).await;
    }
}

