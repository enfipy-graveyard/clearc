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

impl TodoUsecase {}
