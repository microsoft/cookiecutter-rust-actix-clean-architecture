use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Clone)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
}
