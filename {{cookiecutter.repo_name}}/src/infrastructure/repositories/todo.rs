use std::process::id;
use std::sync::Arc;
use actix_threadpool::run;
use async_trait::async_trait;
use diesel::internal::operators_macro::FieldAliasMapper;
use diesel::prelude::*;

use crate::domain::models::todo::{CreateTodo, Todo};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::domain::repositories::todo::{TodoQueryParams, TodoRepository};
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::models::todo::{CreateTodoDiesel, TodoDiesel};

pub struct TodoDieselRepository {
    pub pool: Arc<DBConn>
}

impl TodoDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        TodoDieselRepository { pool: db }
    }
}

#[async_trait]
impl TodoRepository for TodoDieselRepository {

    async fn create(&self, new_todo: &CreateTodo) -> RepositoryResult<Todo> {
        use crate::infrastructure::schema::todos::dsl::todos;
        let new_todo_diesel: CreateTodoDiesel = CreateTodoDiesel::from(new_todo.clone());
        let mut conn = self.pool.get().unwrap();
        let result: TodoDiesel = run(move || diesel::insert_into(todos).values(new_todo_diesel)
            .get_result(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(result.into())
    }

    async fn list(&self, params: TodoQueryParams) -> RepositoryResult<ResultPaging<Todo>> {
        use crate::infrastructure::schema::todos::dsl::todos;
        let pool = self.pool.clone();
        let builder = todos.limit(params.limit()).offset(params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<TodoDiesel>(&mut conn)
        })
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(ResultPaging {
            total: 0,
            items: result.into_iter().map(|v| v.into()).collect()
        })
    }

    // async fn get(&self, todo_id: i32) -> RepositoryResult<Todo> {
    //     use crate::infrastructure::schema::todos::dsl::{id, todos};
    //     let mut conn = self.pool.get().unwrap();
    //     let id_filter = todo_id.to_string();
    //     run(move || todos.filter(id.eq(id_filter)).first::<TodoDiesel>(&mut conn))
    //         .await
    //         .map_err(|v| DieselRepositoryError::from(v).into_inner())
    //         .map(|v| -> Todo { v.into() })
    // }
    //
    // async fn delete(&self, todo_id: i32) -> RepositoryResult<()> {
    //     use crate::infrastructure::schema::todos::dsl::{id, todos};
    //     let mut conn = self
    //         .pool
    //         .get()
    //         .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
    //     let id_filter = todo_id.to_string();
    //     run(move || diesel::delete(todos).filter(id.eq(id_filter))
    //         .execute(&mut conn))
    //         .await
    //         .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
    //     Ok(())
    // }
}