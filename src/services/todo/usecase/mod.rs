use crate::config::Config;
use crate::helpers::database::Database;
use crate::models::todo;

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

impl TodoUsecase {
    pub fn add_todo(&self, description: String) -> todo::Todo {
        let client = self.db_pool.get().expect("Failed to get connection");
        let todo = todo::Todo {
            id: uuid::Uuid::new_v4(),
            description: description,
            completed: false,
        };
        client
            .execute(
                "INSERT INTO todos VALUES($1, $2, $3)",
                &[&todo.id, &todo.description, &todo.completed],
            )
            .expect("Failed to add todo");
        todo
    }
}
