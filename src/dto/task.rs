use std::time::SystemTime;

use crate::{dto::IdentifierInput, util::epoch};

#[derive(Debug, Deserialize)]
pub(crate) struct CreateInput {
    pub(crate) state: IdentifierInput,
    pub(crate) title: String,
    pub(crate) description: String,
}

#[derive(Debug, Serialize, Queryable)]
pub(crate) struct TaskOutput {
    pub(crate) nr: i32,
    pub(crate) progress: i32,
    pub(crate) created_by: String,
    pub(crate) taken_by: Option<String>,
    #[serde(serialize_with = "epoch::system_time")]
    pub(crate) created_at: SystemTime,
    #[serde(serialize_with = "epoch::option_system_time")]
    pub(crate) taken_at: Option<SystemTime>,
    #[serde(serialize_with = "epoch::option_system_time")]
    pub(crate) completed_at: Option<SystemTime>,
    pub(crate) title: String,
    pub(crate) description: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UpdateTaskInput {
    pub(crate) progress: Option<i32>,
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
}

#[derive(Debug)]
pub(crate) struct SearchTaskInput<'a> {
    pub(crate) progress: Option<&'a String>,
    pub(crate) created_by: Option<&'a String>,
    pub(crate) taken_by: Option<&'a String>,
    pub(crate) title: Option<&'a String>,
    pub(crate) description: Option<&'a String>,
}
