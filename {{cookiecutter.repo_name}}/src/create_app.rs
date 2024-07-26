use std::sync::Arc;
use actix_web::{App, web};
use actix_web::{Error};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use crate::api::controllers::todo_handler::{create_todo_handler, delete_todo_handler, get_todo_handler, list_todos_handler};
use crate::api::middleware::{ServiceContextMaintenanceCheck};
use crate::container::Container;

pub fn create_app(container: Arc<Container>) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let todo_service = container.todo_service.clone();
    let service_context_service = container.service_context_service.clone();

    App::new()
        .app_data(web::Data::from(todo_service.clone()))
        .app_data(web::Data::from(service_context_service.clone()))
        .wrap(Logger::default())
        .wrap(ServiceContextMaintenanceCheck)
        .service(
            web::scope("/todos")
                .route("", web::post().to(create_todo_handler))
                .route("", web::get().to(list_todos_handler))
                .route("/{id}", web::get().to(get_todo_handler))
                .route("/{id}", web::delete().to(delete_todo_handler))
        )
}
