use uuid::Uuid;

use crate::{
    config::db::postgres::PgPool,
    dto::state::CreateInput,
    error::{Error, Result},
    model::state::{CreateStateData, State, UpdateStateData},
};

pub(crate) struct StateService;

impl StateService {
    pub(crate) async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<State> {
        State::find_by_id(id, pool).await
    }

    pub(crate) async fn find_by_progress(progress: i32, pool: &PgPool) -> Result<State> {
        State::find_by_progress(progress, pool).await
    }

    pub(crate) async fn find_by_name(name: &str, pool: &PgPool) -> Result<State> {
        State::find_by_name(name, pool).await
    }

    pub(crate) async fn create(input: CreateInput, pool: &PgPool) -> Result<State> {
        if State::find_by_name(&input.name, pool).await.is_ok() {
            return Err(Error::DuplicateStateName);
        }

        if State::find_by_progress(input.progress, pool).await.is_ok() {
            return Err(Error::DuplicateStateName);
        }

        let data = CreateStateData {
            name: input.name,
            description: input.description,
            progress: input.progress,
        };

        State::create(data, pool).await
    }

    pub(crate) async fn update(old: State, input: UpdateStateData, pool: &PgPool) -> Result<State> {
        let name = match input.name {
            Some(name) if name != old.name => {
                if State::find_by_name(&name, pool).await.is_ok() {
                    return Err(Error::DuplicateStateName);
                } else {
                    Some(name)
                }
            }
            Some(_) => None,
            None => None,
        };

        let description = match input.description {
            Some(description) if description != old.description => Some(description),
            Some(_) => None,
            None => None,
        };

        let progress = match input.progress {
            Some(progress) if progress != old.progress => {
                if State::find_by_progress(progress, pool).await.is_ok() {
                    return Err(Error::DuplicateStateProgress);
                } else {
                    Some(progress)
                }
            }
            Some(_) => None,
            None => None,
        };

        if name.is_none() && description.is_none() && progress.is_none() {
            return Ok(old);
        }

        let data = UpdateStateData {
            name,
            description,
            progress,
        };

        State::update(old.id, data, pool).await
    }

    pub(crate) async fn list(pool: &PgPool) -> Result<Vec<State>> {
        State::get_all(pool).await
    }
}
