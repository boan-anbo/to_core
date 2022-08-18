use crate::db::db_op::reset_database;
use crate::to_machine::to_machine_struct::TextualObjectMachine;

pub mod to_machine_struct;
pub mod to_machine_option;
pub mod to_machine_op;
pub mod to_machine_db;

// implement db related methods for TextualObjectMachine
impl TextualObjectMachine {
    // clear all tables;
    pub async fn reset_db(&self) -> () {
        reset_database(self.store_path.as_ref()).await;
    }
}

