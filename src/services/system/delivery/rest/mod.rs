use crate::config::Config;
use crate::services::system::controller::SystemController;

use actix_web::{web, HttpResponse, Scope};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SystemRest {
    pub cnfg: Arc<Config>,
    pub system_cnr: Arc<SystemController>,
}

pub fn init(cnfg: &Arc<Config>, system_cnr: &Arc<SystemController>) -> Scope {
    let system = SystemRest {
        cnfg: cnfg.clone(),
        system_cnr: system_cnr.clone(),
    };
    web::scope("/system")
        .data(system)
        .route("/time", web::get().to(index))
}

fn index(data: web::Data<SystemRest>) -> HttpResponse {
    let (server_time, database_time) = data.system_cnr.system_time();
    let res = format!(
        "Server time: {}\nDatabase time: {}",
        server_time, database_time,
    );
    HttpResponse::Ok().body(res)
}
