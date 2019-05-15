use crate::config::Config;
use crate::services::todo::usecase::TodoUsecase;

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TodoController {
    pub cnfg: Arc<Config>,
    pub todo_ucs: Arc<TodoUsecase>,
}

pub fn init(cnfg: &Arc<Config>, todo_ucs: &Arc<TodoUsecase>) -> Arc<TodoController> {
    let cnr = TodoController {
        cnfg: cnfg.clone(),
        todo_ucs: todo_ucs.clone(),
    };
    Arc::new(cnr)
}

impl TodoController {
    pub fn index(&self) -> String {
        let data = self.todo_ucs.index();
        format!(
            "Server port is {} with data \"{}\"",
            self.cnfg.server_port, data
        )
    }
}
