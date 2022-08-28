use crate::db::db_op::drop_database;
use crate::to_machine::to_machine_struct::ToMachine;

impl ToMachine {
    pub(crate) async fn delete_store(&self) {
        if self.pool.is_some() {
            self.close_pool().await;
        }
        // check if pool is closed, if not, close it
        if !self.pool.as_ref().unwrap().is_closed() {
            panic!("Pool is not closed");
        }
        let drop_result = drop_database(self.store_url.as_str()).await;
        if drop_result.is_err() {
            panic!("Cannot drop database at {}", self.store_url);
        }
    }

    pub(crate) async fn close_pool(&self) {
        self.pool.as_ref().unwrap().close().await;
    }
}
