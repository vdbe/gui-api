use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    config::db::postgres::PgPool,
    dto::task::{CreateInput, TaskOutput, UpdateTaskInput},
    error::{Error, Result},
    model::{
        state::State,
        task::{CreateTaskData, Task, UpdateTaskData},
    },
};

pub(crate) struct TaskService;

impl TaskService {
    pub(crate) async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<TaskOutput> {
        TaskOutput::find_by_id(id, pool).await
    }

    pub(crate) async fn find_by_nr(nr: i32, pool: &PgPool) -> Result<TaskOutput> {
        TaskOutput::find_by_nr(nr, pool).await
    }

    pub(crate) async fn list(pool: &PgPool) -> Result<Vec<TaskOutput>> {
        TaskOutput::get_all(pool).await
    }

    pub(crate) async fn create(
        created_by: Uuid,
        input: CreateInput,
        pool: &PgPool,
    ) -> Result<TaskOutput> {
        let now = OffsetDateTime::now_utc().into();

        let state = match State::find_id(input.state, pool).await {
            Ok(state) => state,
            Err(Error::DieselResult(diesel::result::Error::NotFound)) => {
                return Err(Error::ProgressStateNotFound)
            }
            Err(err) => return Err(err),
        };

        let data = CreateTaskData {
            state,
            created_by,
            created_at: now,
            updated_at: now,
            title: input.title,
            description: input.description,
        };

        TaskOutput::find_by_id(Task::create(data, pool).await?.id, pool).await
    }

    pub(crate) async fn update(
        old: Task,
        input: UpdateTaskInput,
        pool: &PgPool,
    ) -> Result<TaskOutput> {
        let title = match input.title {
            Some(title) if title != old.title => Some(title),
            Some(_) => None,
            None => None,
        };

        let description = match input.description {
            Some(description) if description != old.description => Some(description),
            Some(_) => None,
            None => None,
        };

        let state = match input.progress {
            Some(progress) => match State::find_by_progress(progress, pool).await {
                Ok(state) if state.id != old.state => Some(state.id),
                Ok(_) => None,
                Err(Error::DieselResult(diesel::result::Error::NotFound)) => {
                    return Err(Error::ProgressStateNotFound)
                }
                Err(err) => return Err(err),
            },
            None => None,
        };

        if title.is_none() && description.is_none() && state.is_none() {
            return TaskOutput::find_by_id(old.id, pool).await;
        }

        let data = UpdateTaskData {
            state,
            created_by: None,
            taken_by: None,
            created_at: None,
            updated_at: Some(OffsetDateTime::now_utc().into()),
            taken_at: None,
            completed_at: None,
            title,
            description,
        };

        TaskOutput::update(old.id, data, pool).await
    }
}
