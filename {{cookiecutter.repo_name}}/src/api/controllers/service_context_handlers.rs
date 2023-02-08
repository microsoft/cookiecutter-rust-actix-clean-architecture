//
//
//
// pub async fn update_service_context_handler(
//     service_context_service: web::Data<dyn ServiceContextService>, post_data: web::Json<ServiceContextDTO>,
// ) -> Result<web::Json<ServiceContextDTO>, ApiError> {
//     let service_context = service_context_service.update(post_data.into_inner().into()).await?;
//     Ok(web::Json(service_context.into()))
// }
//
// pub async fn get_service_context(
//     service_context_service: web::Data<dyn ServiceContextService>,
// ) -> Result<web::Json<ServiceContextDTO>, ApiError> {
//     let service_context = service_context_service.get_service_context().await?;
//     Ok(web::Json(service_context.into()))
// }