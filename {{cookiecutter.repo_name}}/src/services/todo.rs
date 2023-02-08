use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::todo::{CreateTodo, Todo};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::todo::{TodoQueryParams, TodoRepository};
use crate::domain::services::todo::TodoService;

#[derive(Clone)]
pub struct TodoServiceImpl {
    pub repository: Arc<dyn TodoRepository>,
}

impl TodoServiceImpl {
    pub fn new(repository: Arc<dyn TodoRepository>) -> Self {
        TodoServiceImpl {
            repository,
        }
    }
}

#[async_trait]
impl TodoService for TodoServiceImpl {
    async fn create(&self, todo: CreateTodo) -> Result<Todo, CommonError> {
        let mut cloned = todo.clone();
        self.repository
            .create(&mut cloned)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn list(&self, params: TodoQueryParams) -> Result<ResultPaging<Todo>, CommonError> {
        self.repository
            .list(params)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn get(&self, todo_id: i32) -> Result<Todo, CommonError> {
        self.repository
            .get(todo_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }

    async fn delete(&self, todo_id: i32) -> Result<(), CommonError> {
        self.repository
            .delete(todo_id)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
