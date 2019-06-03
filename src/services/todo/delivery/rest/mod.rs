use crate::config::Config;
use crate::services::todo::controller::TodoController;

use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
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
        .route("/info", web::get().to(info))
        .route("/send", web::get().to(send_mail))
        .route("/add", web::post().to(add_todo))
        .route("/complete", web::post().to(complete_todo))
}

fn info(data: web::Data<TodoRest>) -> HttpResponse {
    let info = data.todo_cnr.todo_info();
    let res = format!("Todo info: {}", info);
    HttpResponse::Ok().body(res)
}

fn send_mail(data: web::Data<TodoRest>) -> HttpResponse {
    data.todo_cnr.send_mail(
        String::from("example@gmail.com"),
        String::from("d-18bff6089988464ba6510126d81c80c2"),
    );
    HttpResponse::Ok().body("Mail sent")
}

#[derive(Debug, Serialize, Deserialize)]
struct AddTodoReq {
    description: String,
}

fn add_todo(req: web::Json<AddTodoReq>, data: web::Data<TodoRest>) -> HttpResponse {
    let id = data.todo_cnr.add_todo(req.description.clone());
    let res = format!("Todo added: {}", id);
    HttpResponse::Ok().body(res)
}

#[derive(Debug, Serialize, Deserialize)]
struct CompleteTodoReq {
    id: uuid::Uuid,
}

fn complete_todo(req: web::Json<CompleteTodoReq>, data: web::Data<TodoRest>) -> HttpResponse {
    data.todo_cnr.complete_todo(req.id);
    HttpResponse::Ok().body("Todo completed")
}
