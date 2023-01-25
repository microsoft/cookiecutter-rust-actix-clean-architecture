use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::todo::{CreateTodo, Todo};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::todo::TodoQueryParams;

#[async_trait]
pub trait TodoService: Sync + Send {
    async fn create(&self, todo: CreateTodo) -> Result<Todo, CommonError>;
    async fn list(&self, params: TodoQueryParams) -> Result<ResultPaging<Todo>, CommonError>;
    // async fn get(&self, todo_id: i32) -> Result<Todo, CommonError>;
    // async fn delete(&self, todo_id: i32) -> Result<(), CommonError>;
}

