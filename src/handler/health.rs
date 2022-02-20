use axum::{extract::Extension, routing::get, Json, Router};

use crate::{
    config::db::postgres::PgPool, error::ApiResult, model::health::Health, service::HealthService,
};

pub(crate) fn routes() -> Router {
    Router::new().route("/", get(health))
}

pub(crate) async fn health(Extension(pool): Extension<PgPool>) -> ApiResult<Json<Health>> {
    Ok(Json(HealthService::get(&pool).await?))
}
