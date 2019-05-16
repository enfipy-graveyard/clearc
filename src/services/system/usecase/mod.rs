use crate::config::Config;
use crate::helpers::database::Database;

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SystemUsecase {
    pub cnfg: Arc<Config>,
    pub db_pool: Database,
}

pub fn init(cnfg: &Arc<Config>, db_pool: &Database) -> Arc<SystemUsecase> {
    let cnr = SystemUsecase {
        cnfg: cnfg.clone(),
        db_pool: db_pool.clone(),
    };
    Arc::new(cnr)
}

impl SystemUsecase {
    pub fn database_time(&self) -> i64 {
        let client = self.db_pool.get().expect("Failed to get connection");
        let mut database_time: i64 = 0;
        for row in &client
            .query("SELECT now()::INT", &[])
            .expect("Failed to retrieve database time")
        {
            database_time = row.get(0);
        }
        database_time
    }
}
