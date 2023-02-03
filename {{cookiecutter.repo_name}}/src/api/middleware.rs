use std::future::{ready, Ready};

use actix_web::{body::EitherBody, dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse, web};
use futures_util::future::LocalBoxFuture;
use log::info;
use crate::domain::services::service_context::ServiceContextService;

pub struct ServiceContextMaintenanceCheck;

impl<S, B> Transform<S, ServiceRequest> for ServiceContextMaintenanceCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ServiceContextMaintenanceCheckMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ServiceContextMaintenanceCheckMiddleware { service }))
    }
}
pub struct ServiceContextMaintenanceCheckMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ServiceContextMaintenanceCheckMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let service_context_service =
            request.app_data::<web::Data<dyn ServiceContextService>>().unwrap();

        if service_context_service.is_maintenance_active() {
            info!("Service is in maintenance mode");
            let (request, _pl) = request.into_parts();
            let response = HttpResponse::ServiceUnavailable().finish().map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(request);
        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}