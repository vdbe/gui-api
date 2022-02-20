use std::time::SystemTime;

use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    config::db::postgres::PgPool,
    dto::task::CreateInput,
    error::{Error, Result},
    model::{
        state::State,
        task::{CreateTaskData, Task},
    },
};

pub(crate) struct TaskService;

impl TaskService {
    pub(crate) async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Task> {
        Task::find_by_id(id, pool).await
    }

    pub(crate) async fn find_by_title(title: &str, pool: &PgPool) -> Result<Task> {
        Task::find_by_name(title, pool).await
    }

    pub(crate) async fn create(
        created_by: Uuid,
        input: CreateInput,
        pool: &PgPool,
    ) -> Result<Task> {
        let now = OffsetDateTime::now_utc().into();

        let data = CreateTaskData {
            state: State::find_id(input.state, pool).await?,
            created_by,
            created_at: now,
            updated_at: now,
            title: input.title,
            description: input.description,
        };

        Task::create(data, pool).await
    }
}
