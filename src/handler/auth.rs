use axum::{
    extract::Extension,
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};

use crate::{
    config::{constant::BEARER, db::postgres::PgPool},
    dto::{
        auth::{LoginInput, RegisterInput, UpdateInput},
        TokenPayload,
    },
    error::{ApiResult, Error},
    model::User,
    service::AuthService,
    util::{
        jwt::{self, Claims},
        validate_payload,
    },
};

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/update", patch(update))
        .route("/authorize", get(authorize))
        .route("/claims", get(claims))
}

pub(crate) async fn authorize(user: User) -> ApiResult<Json<User>> {
    Ok(Json(user))
}

pub(crate) async fn claims(claims: Claims) -> ApiResult<Json<Claims>> {
    Ok(Json(claims))
}

pub(crate) async fn login(
    Json(input): Json<LoginInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<TokenPayload>> {
    validate_payload(&input)?;

    let user = AuthService::sign_in(input, &pool)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    let token = jwt::sign(user.id)?;

    Ok(Json(TokenPayload {
        access_token: token,
        token_type: BEARER.to_string(),
    }))
}

pub(crate) async fn register(
    Json(input): Json<RegisterInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<(StatusCode, Json<TokenPayload>)> {
    validate_payload(&input)?;

    let user = AuthService::sign_up(input, &pool).await?;
    let token = jwt::sign(user.id)?;

    Ok((
        StatusCode::CREATED,
        Json(TokenPayload {
            access_token: token,
            token_type: BEARER.to_string(),
        }),
    ))
}

pub(crate) async fn update(
    user: User,
    Json(input): Json<UpdateInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<TokenPayload>> {
    validate_payload(&input)?;

    let user = AuthService::update(user, input, &pool).await?;
    let token = jwt::sign(user.id)?;

    Ok(Json(TokenPayload {
        access_token: token,
        token_type: BEARER.to_string(),
    }))
}
