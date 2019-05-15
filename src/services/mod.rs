pub mod todo;

use crate::config::Config;
use crate::helpers::database;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;

pub fn init_services(cnfg: Arc<Config>) {
    let db_pool = database::init_pool(&cnfg, 5).expect("Failed to init database connection");

    let app = move || {
        let todo_ucs = todo::usecase::init(&cnfg, &db_pool);
        let todo_cnr = todo::controller::init(&cnfg, &todo_ucs);
        let todo_dlr_rest = todo::delivery::rest::init(&cnfg, &todo_cnr);

        let api = web::scope("/api").service(todo_dlr_rest);

        App::new().service(api)
    };

    HttpServer::new(app)
        .bind("0.0.0.0:8000")
        .expect("Failed to bind port for the http server")
        .run()
        .expect("Failed to run http server");
}
