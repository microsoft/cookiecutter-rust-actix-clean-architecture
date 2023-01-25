use std::sync::Arc;
use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use actix_web::web::route;
use actix_clean_architecture::api::controllers::todo_handler::{create_todo_handler, list_todos_handler};
use actix_clean_architecture::container::Container;
use actix_clean_architecture::domain::repositories::todo::TodoRepository;
use actix_clean_architecture::domain::services::todo::TodoService;
use actix_clean_architecture::infrastructure::databases::postgresql::db_pool;
use actix_clean_architecture::infrastructure::repositories::todo::TodoDieselRepository;
use actix_clean_architecture::services::todo::TodoServiceImpl;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let container = Container::new();
    let todo_service = container.todo_service.clone();
    let todo_service_data = web::Data::from(todo_service.clone());
    let server = HttpServer::new(move || {
        App::new()
            .app_data(todo_service_data.clone())
            .service(
                web::scope("/todos")
                    .route("", web::post().to(create_todo_handler))
                    .route("", web::get().to(list_todos_handler))
            )
    })
    .bind(("127.0.0.1", 8080))?;
    server.run().await
}

