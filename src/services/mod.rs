pub mod system;
pub mod todo;

use crate::config::Config;
use crate::helpers::database;
use crate::helpers::email;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;

pub fn init_services(cnfg: Arc<Config>) {
    let db_pool = database::init_pool(&cnfg, 5).expect("Failed to init database connection");
    let mailer = email::init_mailer(&cnfg);

    let system_ucs = system::usecase::init(&cnfg, &db_pool);
    let todo_ucs = todo::usecase::init(&cnfg, &db_pool);

    let system_cnr = system::controller::init(&cnfg, &system_ucs);
    let todo_cnr = todo::controller::init(&cnfg, &todo_ucs, &system_ucs, &mailer);

    let app = move || {
        let system_dlr_rest = system::delivery::rest::init(&cnfg, &system_cnr);
        let todo_dlr_rest = todo::delivery::rest::init(&cnfg, &todo_cnr);

        let api = web::scope("/api/v1")
            .service(system_dlr_rest)
            .service(todo_dlr_rest);

        App::new().service(api)
    };

    // Todo: Move to main file
    HttpServer::new(app)
        .bind("0.0.0.0:8000")
        .expect("Failed to bind port for the http server")
        .run()
        .expect("Failed to run http server");
}
