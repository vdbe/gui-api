use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use crate::{
    config::db::postgres::PgPool,
    dto::{state::CreateInput, IdentifierPath},
    error::ApiResult,
    model::state::{State, UpdateStateData},
    service::StateService,
    util::jwt::Claims,
};

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:identifier", get(find_by).patch(update))
}
pub(crate) async fn create(
    _: Claims,
    Json(input): Json<CreateInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<(StatusCode, Json<State>)> {
    let state = StateService::create(input, &pool).await?;

    Ok((StatusCode::CREATED, Json(state)))
}

pub(crate) async fn list(
    _: Claims,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<Vec<State>>> {
    Ok(Json(StateService::list(&pool).await?))
}

pub(crate) async fn find_by(
    _: Claims,
    Path(identifier): Path<IdentifierPath>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<State>> {
    let state = match identifier {
        IdentifierPath::Integer(p) => StateService::find_by_progress(p, &pool).await?,
        IdentifierPath::Text(n) => StateService::find_by_name(&n, &pool).await?,
    };

    Ok(Json(state))
}

pub(crate) async fn update(
    _: Claims,
    Path(identifier): Path<IdentifierPath>,
    Json(input): Json<UpdateStateData>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<State>> {
    let state = match identifier {
        IdentifierPath::Integer(p) => StateService::find_by_progress(p, &pool).await?,
        IdentifierPath::Text(n) => StateService::find_by_name(&n, &pool).await?,
    };

    Ok(Json(StateService::update(state, input, &pool).await?))
}
