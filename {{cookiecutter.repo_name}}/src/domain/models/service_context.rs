use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct ServiceContext {
    pub id: i32,
    pub maintenance: bool,
}
