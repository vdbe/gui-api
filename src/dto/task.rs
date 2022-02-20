use std::time::SystemTime;

use crate::dto::state::IdentifierInput as StateIdentifierInput;

#[derive(Debug, Deserialize)]
pub(crate) struct CreateInput {
    pub(crate) state: StateIdentifierInput,
    pub(crate) title: String,
    pub(crate) description: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Output {
    pub(crate) progress: i32,
    pub(crate) create_by: String,
    pub(crate) taken_by: Option<String>,
    pub(crate) created_at: SystemTime,
    pub(crate) taken_at: Option<SystemTime>,
    pub(crate) completed_at: Option<SystemTime>,
    pub(crate) title: String,
    pub(crate) description: String,
}
