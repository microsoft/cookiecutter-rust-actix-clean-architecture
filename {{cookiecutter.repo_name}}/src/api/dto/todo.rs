use crate::domain::models::todo::{CreateTodo, Todo};
use serde::{Serialize, Deserialize};
use crate::domain::repositories::repository::ResultPaging;

#[derive(Deserialize, Serialize)]
pub struct CreateTodoDTO {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct TodoDTO {
    id: i32,
    title: String,
    description: String,
    completed: bool,
}

impl Into<TodoDTO> for Todo {
    fn into(self) -> TodoDTO {
        TodoDTO {
            id: self.id,
            title: self.title,
            description: self.description,
            completed: false
        }
    }
}

impl Into<CreateTodo> for CreateTodoDTO {
    fn into(self) -> CreateTodo {
        CreateTodo {
            title: self.title,
            description: self.description,
        }
    }
}

impl Into<CreateTodoDTO> for CreateTodo {
    fn into(self) -> CreateTodoDTO {
        CreateTodoDTO {
            title: self.title,
            description: self.description,
        }
    }
}

impl Into<ResultPaging<TodoDTO>> for ResultPaging<Todo> {
    fn into(self) -> ResultPaging<TodoDTO> {
        ResultPaging {
            total: self.total,
            items: self.items.into_iter().map(|todo| todo.into()).collect(),
        }
    }
}