use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::{
    config::db::postgres::PgPool,
    dto::task::CreateInput,
    error::ApiResult,
    model::{task::Task, User},
    service::TaskService,
    util::jwt::Claims,
};

pub(crate) fn routes() -> Router {
    Router::new().route("/", post(create))
}

pub(crate) async fn create(
    claims: Claims,
    Json(input): Json<CreateInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<(StatusCode, Json<Task>)> {
    let task = TaskService::create(claims.sub, input, &pool).await?;

    Ok((StatusCode::CREATED, Json(task)))
}
