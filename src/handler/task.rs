use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::{
    config::db::postgres::PgPool,
    dto::{
        task::{CreateInput, TaskOutput, UpdateTaskInput},
        IdentifierPath,
    },
    error::{ApiResult, Error},
    model::task::Task,
    service::TaskService,
    util::jwt::Claims,
};

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", post(create).get(list))
        .route("/:identifier", get(find_by).patch(update))
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
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<Vec<TaskOutput>>> {
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
