use crate::config::Config;
use crate::services::system::usecase::SystemUsecase;

use chrono::Utc;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SystemController {
    pub cnfg: Arc<Config>,
    pub system_ucs: Arc<SystemUsecase>,
}

pub fn init(cnfg: &Arc<Config>, system_ucs: &Arc<SystemUsecase>) -> Arc<SystemController> {
    let cnr = SystemController {
        cnfg: cnfg.clone(),
        system_ucs: system_ucs.clone(),
    };
    Arc::new(cnr)
}

impl SystemController {
    pub fn system_time(&self) -> (i64, i64) {
        let server_time = Utc::now().timestamp();
        let database_time = self.system_ucs.database_time();
        (server_time, database_time)
    }
}
