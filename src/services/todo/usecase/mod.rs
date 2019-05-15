use crate::config::Config;
use crate::helpers::database::Database;

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TodoUsecase {
    pub cnfg: Arc<Config>,
    pub db_pool: Database,
}

pub fn init(cnfg: &Arc<Config>, db_pool: &Database) -> Arc<TodoUsecase> {
    let cnr = TodoUsecase {
        cnfg: cnfg.clone(),
        db_pool: db_pool.clone(),
    };
    Arc::new(cnr)
}

impl TodoUsecase {
    pub fn index(&self) -> String {
        let client = self.db_pool.get().expect("Failed to get connection");
        let mut data = String::from("");
        for row in &client
            .query("SELECT 'hello world'", &[])
            .expect("Failed to execute query")
        {
            let row_data: String = row.get(0);
            data += &row_data;
        }
        data
    }
}
