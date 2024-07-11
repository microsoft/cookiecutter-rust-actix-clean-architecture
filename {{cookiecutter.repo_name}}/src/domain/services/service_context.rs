use crate::domain::models::service_context::ServiceContext;

pub trait ServiceContextService: 'static + Sync + Send {
    fn get_service_context(&self) -> ServiceContext;
    fn update(&self, service_context: ServiceContext) -> ServiceContext;
    fn is_maintenance_active(&self) -> bool;
}
