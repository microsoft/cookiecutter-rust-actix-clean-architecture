use diesel;
use diesel::prelude::*;
use crate::domain::models::service_context::ServiceContext;
use crate::infrastructure::schema::service_contexts;


#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = service_contexts)]
pub struct ServiceContextDiesel {
    pub id: i32,
    pub maintenance: bool,
}

impl From<ServiceContextDiesel> for ServiceContext {
    fn from(service_context: ServiceContextDiesel) -> Self {
        ServiceContext {
            id: service_context.id,
            maintenance: service_context.maintenance
        }
    }
}

impl From<ServiceContext> for ServiceContextDiesel {
    fn from(service_context: ServiceContext) -> Self {
        ServiceContextDiesel {
            id: service_context.id,
            maintenance: service_context.maintenance
        }
    }
}