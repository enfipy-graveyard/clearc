use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,
}
