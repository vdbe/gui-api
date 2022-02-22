use std::time::SystemTime;

use uuid::Uuid;

use crate::schema::tasks;

#[derive(Debug, Queryable)]
pub(crate) struct Task {
    pub(crate) id: Uuid,
    pub(crate) nr: i32,
    pub(crate) state: Uuid,
    pub(crate) created_by: Uuid,
    pub(crate) taken_by: Option<Uuid>,
    pub(crate) created_at: SystemTime,
    pub(crate) updated_at: SystemTime,
    pub(crate) taken_at: Option<SystemTime>,
    pub(crate) completed_at: Option<SystemTime>,
    pub(crate) title: String,
    pub(crate) description: String,
}

#[derive(Debug, Insertable)]
#[table_name = "tasks"]
pub(crate) struct CreateTaskData {
    pub(crate) state: Uuid,
    pub(crate) created_by: Uuid,
    pub(crate) created_at: SystemTime,
    pub(crate) updated_at: SystemTime,
    pub(crate) title: String,
    pub(crate) description: String,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "tasks"]
pub(crate) struct UpdateTaskData {
    pub(crate) state: Option<Uuid>,
    pub(crate) created_by: Option<Uuid>,
    pub(crate) taken_by: Option<Uuid>,
    pub(crate) created_at: Option<SystemTime>,
    pub(crate) updated_at: Option<SystemTime>,
    pub(crate) taken_at: Option<Option<SystemTime>>,
    pub(crate) completed_at: Option<Option<SystemTime>>,
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
}
