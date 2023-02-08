# Cookiecutter actix simple clean architecture
This is a reusable Rust Cookiecutter template. The project is based on Actix web in combination with Diesel ORM.

Complete list of features the template provides:
* Onion architecture
* Actix Web 
* Maintenance window support
* Diesel ORM
* Database migrations
* Local postgres database docker support
* Test containers integration for testing

## Getting started
To start a new project, run the following command:
```bash
cookiecutter -c v1 https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture
```
This will prompt you for some information about your project. The information
you provide will be used to populate the files in the new project directory.

You can then build the project locally.
```bash
cargo build
```

## Architecture 
The application follows the Onion Architecture pattern. An article is written 
about our experience integrating an onion architecture with actix web in combination with diesel ORM that can 
be found [here](./docs/onion-architecture-article.md).

This architecture is a design pattern that organizes the codebase of a software application into multiple layers, where the innermost layer 
is the domain layer and the outermost layer is the application layer. Each layer depends only on the layers inside of it and not on the layers outside of it, 
creating a separation of concerns, allowing for a more maintainable and scalable codebase.

For this template we suggest using a service-repository design pattern. For example implementations you can have a look at 


## Running the application locally
To run the application locally, you need to have a Postgres database running.
You can use the `run_postgres.sh` script in the `scripts` directory to run a Postgres container.
```bash
./scripts/run_postgres.sh
```

You can then run the application.
```bash
cargo run
```

## Testing support
All tests are can be found under the `src/tests` folder. When using the template
you can place all you tests in this folder.

To run the tests, you can use the following command:
```bash
cargo test
```
To run the tests with error output you can run the following command:
```bash
cargo test -- --nocapture
```
or 
```bash
cargo test -- --show-output
```

## Diesel ORM
The template uses Diesel ORM for its database connection and database models
integration. Its is currently setup with postgres, however you can 
change it to any other database that is supported by diesel. For other databases 
have a look at the official Diesel documentation that can be found [here](https://diesel.rs/)

### Database migrations

1) Make sure you have the diesel cli installed. You can install it with the following command:
    ```bash
    cargo install diesel_cli --no-default-features --features postgres
    ```
2) Add your postgres database url to the .env file:
    ```bash
    echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
    ```
3) Setup diesel before creating a migration:
    ```bash
    diesel setup
    ```
4) Create a migration with the following command:
    ```bash
    diesel migration generate <migration_name>
    ```
5) Apply your migrations:
    ```bash
    diesel migration run
    ```

## Service repository design pattern

### Diesel Repositories
The onion architecture is best being used with a repository-service pattern. An example 
repository can be seen below:

```rust
// Can be placed under /src/domain/repositories/todo.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
}

impl QueryParams for TodoQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, new_todo: &CreateTodo) -> RepositoryResult<Todo>;
    async fn list(&self, params: TodoQueryParams) -> RepositoryResult<ResultPaging<Todo>>;
    async fn get(&self, todo_id: i32) -> RepositoryResult<Todo>;
    async fn delete(&self, todo_id: i32) -> RepositoryResult<()>;
}
```

```rust
// Can be placed under /src/infrastructure/repositories/todo.rs
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

    async fn get(&self, todo_id: i32) -> RepositoryResult<Todo> {
        use crate::infrastructure::schema::todos::dsl::{id, todos};
        let mut conn = self.pool.get().unwrap();
        run(move || todos.filter(id.eq(todo_id)).first::<TodoDiesel>(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())
            .map(|v| -> Todo { v.into() })
    }

    async fn delete(&self, todo_id: i32) -> RepositoryResult<()> {
        use crate::infrastructure::schema::todos::dsl::{id, todos};
        let mut conn = self.pool.get().unwrap();
        run(move || diesel::delete(todos).filter(id.eq(todo_id))
            .execute(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(())
    }
}
```

### Services
The onion architecture is best being used with a repository-service pattern. An example 
service can be seen below:
```rust
// Can be placed under /src/services/todo.rs
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
```
