use crate::config::Config;
use crate::helpers::email::Mailer;
use crate::services::system::usecase::SystemUsecase;
use crate::services::todo::usecase::TodoUsecase;

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TodoController {
    pub cnfg: Arc<Config>,
    pub todo_ucs: Arc<TodoUsecase>,
    pub system_ucs: Arc<SystemUsecase>,
    pub mailer: Arc<Mailer>,
}

pub fn init(
    cnfg: &Arc<Config>,
    todo_ucs: &Arc<TodoUsecase>,
    system_ucs: &Arc<SystemUsecase>,
    mailer: &Arc<Mailer>,
) -> Arc<TodoController> {
    let cnr = TodoController {
        cnfg: cnfg.clone(),
        todo_ucs: todo_ucs.clone(),
        system_ucs: system_ucs.clone(),
        mailer: mailer.clone(),
    };
    Arc::new(cnr)
}

impl TodoController {
    pub fn todo_info(&self) -> i64 {
        let database_time = self.system_ucs.database_time();
        database_time
    }

    pub fn send_mail(&self, email: String, template_id: String) {
        self.mailer
            .send(email, template_id)
            .expect("Failed to send mail");
    }
}
