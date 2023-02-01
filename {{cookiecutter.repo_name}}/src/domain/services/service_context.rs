use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::service_context::ServiceContext;

#[async_trait]
pub trait ServiceContextService: Sync + Send {
    async fn get_status(&self) -> Result<ServiceContext, CommonError>;
    async fn update(&self) -> Result<ServiceContext, CommonError>;
}
