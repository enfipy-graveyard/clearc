use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: Uuid,
    pub description: String,
    pub status: TodoStatus,
}

#[derive(Clone, Debug)]
pub enum TodoStatus {
    Active = 0,
    Completed = 1,
}
