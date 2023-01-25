use diesel;
use diesel::prelude::*;
use crate::domain::models::todo::{CreateTodo, Todo};
use crate::infrastructure::schema::todos;

#[derive(Queryable)]
pub struct TodoDiesel {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

// Factory method for creating a new TodoDiesel from a Todo
impl From<Todo> for TodoDiesel {
    fn from(t: Todo) -> Self {
        TodoDiesel {
            id: t.id,
            title: t.title,
            description: t.description,
            completed: t.completed,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct CreateTodoDiesel {
    pub title: String,
    pub description: String,
}

// Factory method for creating a new Todo from a TodoDiesel
impl Into<Todo> for TodoDiesel {
    fn into(self) -> Todo {
        Todo {
            id: self.id,
            title: self.title,
            description: self.description,
            completed: self.completed,
        }
    }
}

impl From<CreateTodo> for CreateTodoDiesel {
    fn from(t: CreateTodo) -> Self {
        CreateTodoDiesel {
            title: t.title,
            description: t.description,
        }
    }
}

impl Into<Todo> for CreateTodoDiesel {
    fn into(self) -> Todo {
        Todo {
            id: 0,
            title: self.title,
            description: self.description,
            completed: false,
        }
    }
}