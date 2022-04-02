#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;

use axum::{extract::Extension, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[cfg(debug_assertions)]
use tower_http::cors::CorsLayer;

use config::db::postgres::PgPool;

pub mod config;
mod dto;
mod error;
mod extractor;
mod handler;
mod model;
mod schema;
mod service;
mod sql;
mod util;

pub fn app(pg_pool: PgPool) -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(Extension(pg_pool));

    #[cfg(debug_assertions)]
    let middleware_stack = middleware_stack.layer(CorsLayer::permissive());

    Router::new()
        .nest("/auth", handler::auth::routes())
        .nest("/state", handler::state::routes())
        .nest("/task", handler::task::routes())
        .nest("/health", handler::health::routes())
        .layer(middleware_stack.into_inner())
}
