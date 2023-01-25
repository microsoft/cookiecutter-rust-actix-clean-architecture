#[cfg(test)]
mod test_todo_controllers{
    use std::env;
    use std::sync::Arc;
    use actix_web::{test, web, App, services, http};
    use actix_web::middleware::Logger;
    use actix_web::web::service;
    use diesel::{Connection, PgConnection};
    use testcontainers::clients;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use testcontainers::images::postgres;
    use actix_clean_architecture::api::dto::todo::CreateTodoDTO;
    use actix_clean_architecture::container::Container;
    use actix_clean_architecture::domain::constants::POSTGRESQL_DB_URI;
    use actix_clean_architecture::domain::repositories::todo::TodoRepository;
    use actix_clean_architecture::infrastructure::databases::postgresql::db_pool;
    use actix_clean_architecture::infrastructure::repositories::todo::TodoDieselRepository;
    use actix_clean_architecture::services::todo::TodoServiceImpl;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use actix_clean_architecture::api::controllers::todo_handler::{create_todo_handler, list_todos_handler};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    #[actix_web::test]
    async fn test() {
        env::set_var("RUST_BACKTRACE", "1");
        env::set_var("RUST_LOG", "debug");
        env::set_var("RUST_BACKTRACE", "1");
        env_logger::init();

        let docker = clients::Cli::default();
        let postgres_node = docker.run(postgres::Postgres::default());
        let connection_string = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres", postgres_node.get_host_port_ipv4(5432)
        );
        env::set_var(POSTGRESQL_DB_URI, connection_string);


        let pool = Arc::new(db_pool());
        pool.get().unwrap().run_pending_migrations(MIGRATIONS).unwrap();


        let container = Container::new();
        let todo_service = container.todo_service.clone();
        let todo_service_data = web::Data::from(todo_service.clone());
        let logger = Logger::default();

        let app = test::init_service(
            App::new()
                .wrap(logger)
                .app_data(todo_service_data.clone())
                .service(
                    web::scope("/todos")
                        .route("", web::get().to(list_todos_handler))
                        .route("", web::post().to(create_todo_handler))
                )
        )
        .await;

        let data = r#"
        {
            "title": "test todo",
            "description": "Test description"
        }"#;

        let request_payload: CreateTodoDTO = serde_json::from_str(data).unwrap();
        let req = test::TestRequest::post().set_json(&request_payload).uri("/todos").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::get().uri("/todos").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}