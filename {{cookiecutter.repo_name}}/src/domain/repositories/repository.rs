use serde::{Deserialize, Serialize};
use crate::domain::error::{RepositoryError};

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultPaging<T> {
    pub total: i64,
    pub items: Vec<T>,
}

pub const DEFAULT_OFFSET: Option<i64> = Some(0);
pub const DEFAULT_LIMIT: Option<i64> = Some(25);

pub trait QueryParams: Send + Sync {
    fn limit(&self) -> i64;
    fn offset(&self) -> i64;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParamsImpl {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl QueryParams for QueryParamsImpl {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}
