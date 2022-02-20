use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::{
    config::db::postgres::PgPool,
    dto::state::{CreateInput, IdentifierInput},
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
    Path(identifier): Path<IdentifierInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<State>> {
    let state = match identifier {
        IdentifierInput::Progress(p) => StateService::find_by_progress(p, &pool).await?,
        IdentifierInput::Name(n) => StateService::find_by_name(&n, &pool).await?,
        IdentifierInput::Id(_) => {
            unreachable!("Deserialize is only implmented for `Progress` and `Name`")
        }
    };

    Ok(Json(state))
}

pub(crate) async fn update(
    _: Claims,
    Path(identifier): Path<IdentifierInput>,
    Json(input): Json<UpdateStateData>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<State>> {
    let state = match identifier {
        IdentifierInput::Progress(p) => StateService::find_by_progress(p, &pool).await?,
        IdentifierInput::Name(n) => StateService::find_by_name(&n, &pool).await?,
        IdentifierInput::Id(_) => {
            unreachable!("Deserialize is only implmented for `Progress` and `Name`")
        }
    };

    Ok(Json(StateService::update(state, input, &pool).await?))
}
