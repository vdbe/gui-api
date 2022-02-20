use std::time::SystemTime;

use uuid::Uuid;

use crate::schema::tasks;

#[derive(Debug, Queryable, Serialize)]
pub(crate) struct Task {
    #[serde(skip)]
    pub(crate) id: Uuid,
    pub(crate) state: Uuid,
    pub(crate) create_by: Uuid,
    pub(crate) taken_by: Option<Uuid>,
    pub(crate) created_at: SystemTime,
    #[serde(skip)]
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
