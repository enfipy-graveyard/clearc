use crate::config::Config;
use crate::services::todo::controller::TodoController;

use actix_web::{web, HttpResponse, Scope};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TodoRest {
    pub cnfg: Arc<Config>,
    pub todo_cnr: Arc<TodoController>,
}

pub fn init(cnfg: &Arc<Config>, todo_cnr: &Arc<TodoController>) -> Scope {
    let todo = TodoRest {
        cnfg: cnfg.clone(),
        todo_cnr: todo_cnr.clone(),
    };
    web::scope("/todo")
        .data(todo)
        .route("/", web::get().to(index))
}

fn index(data: web::Data<TodoRest>) -> HttpResponse {
    let res = data.todo_cnr.index();
    HttpResponse::Ok().body(res)
}
