use std::time::SystemTime;

use uuid::Uuid;

pub(crate) mod auth;
pub(crate) mod health;
pub(crate) mod state;
pub(crate) mod task;

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    #[serde(skip_serializing)]
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) email: String,
    #[serde(skip_serializing)]
    pub(crate) password: String,
    #[serde(skip_serializing)]
    pub(crate) created_at: SystemTime,
    #[serde(skip_serializing)]
    pub(crate) updated_at: SystemTime,
}
