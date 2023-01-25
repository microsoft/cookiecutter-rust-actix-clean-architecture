use std::sync::Arc;
use crate::domain::repositories::todo::TodoRepository;
use crate::domain::services::todo::TodoService;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::repositories::todo::TodoDieselRepository;
use crate::services::todo::TodoServiceImpl;

pub struct Container {
    pub todo_service: Arc<dyn TodoService>,
}

impl Container {
    pub fn new() -> Self {
        let pool = Arc::new(db_pool());
        let todo_repository: Arc<dyn TodoRepository> = Arc::new(
            TodoDieselRepository::new(pool)
        );
        let todo_service = Arc::new(
            TodoServiceImpl { repository: todo_repository }
        );
        Container { todo_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

