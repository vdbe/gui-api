use std::collections::HashMap;

use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::{
    config::db::postgres::PgPool,
    dto::{
        state::PostStateIdentifier,
        task::{CreateInput, SearchTaskInput, TaskOutput, UpdateTaskInput},
        IdentifierPath,
    },
    error::{ApiResult, Error},
    model::{state::State, task::Task},
    service::{StateService, TaskService},
    util::jwt::Claims,
};

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", post(create).get(list))
        .route("/:identifier", get(find_by).patch(update))
        .route("/:identifier/state", get(state).patch(update_state))
}

pub(crate) async fn create(
    claims: Claims,
    Json(input): Json<CreateInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<(StatusCode, Json<TaskOutput>)> {
    let task = TaskService::create(claims.sub, input, &pool).await?;

    Ok((StatusCode::CREATED, Json(task)))
}

pub(crate) async fn list(
    _: Claims,
    Query(params): Query<HashMap<String, String>>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<Vec<TaskOutput>>> {
    if !params.is_empty() {
        let progress = params.get("progress");
        let title = params.get("title");
        let description = params.get("desc").or_else(|| params.get("description"));
        let created_by = params.get("created_by");
        let taken_by = params.get("taken_by");

        if progress.is_some()
            || title.is_some()
            || description.is_some()
            || created_by.is_some()
            || taken_by.is_some()
        {
            let input = SearchTaskInput {
                progress,
                created_by,
                taken_by,
                title,
                description,
            };

            return Ok(Json(TaskService::search(input, &pool).await?));
        }
    }

    Ok(Json(TaskService::list(&pool).await?))
}

pub(crate) async fn find_by(
    _: Claims,
    Path(identifier): Path<IdentifierPath>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<TaskOutput>> {
    let task = match identifier {
        IdentifierPath::Integer(nr) => TaskOutput::find_by_nr(nr, &pool).await?,
        IdentifierPath::Text(_) => return Err(Error::InvalidIdentifier.into()),
    };

    Ok(Json(task))
}

pub(crate) async fn update(
    claims: Claims,
    Path(identifier): Path<IdentifierPath>,
    Json(input): Json<UpdateTaskInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<TaskOutput>> {
    let task = match identifier {
        IdentifierPath::Integer(nr) => Task::find_by_nr(nr, &pool).await?,
        IdentifierPath::Text(_) => return Err(Error::InvalidIdentifier.into()),
    };

    if task.created_by != claims.sub {
        // You can only edit the tasks you created
        return Err(Error::NoEditPermission.into());
    }

    Ok(Json(TaskService::update(task, input, &pool).await?))
}

pub(crate) async fn state(
    _: Claims,
    task: Task,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<State>> {
    Ok(Json(StateService::find_by_id(task.state, &pool).await?))
}

pub(crate) async fn update_state(
    claims: Claims,
    task: Task,
    Json(input): Json<PostStateIdentifier>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<State>> {
    Ok(Json(
        TaskService::update_state(task, input.into(), claims.sub, &pool).await?,
    ))
}
