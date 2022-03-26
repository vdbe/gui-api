use axum::{
    extract::Extension,
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};

use crate::{
    config::{constant::BEARER, db::postgres::PgPool},
    dto::{
        auth::{LoginUserInput, RefreshTokenInput, RegisterUserInput, UpdateUserInput},
        LoginPayload, RefreshPayload,
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
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/token", post(token))
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
    Json(input): Json<LoginUserInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<LoginPayload>> {
    validate_payload(&input)?;

    let user = AuthService::sign_in(input, &pool)
        .await
        .map_err(|_| Error::WrongCredentials)?;
    let token = jwt::sign(user.id)?;
    let refresh_token = AuthService::create_refresh_token(user.id, &pool).await?;

    Ok(Json(LoginPayload {
        refresh_token,
        access_token: RefreshPayload {
            access_token: token,
            token_type: BEARER.to_string(),
        },
    }))
}

pub(crate) async fn logout(
    Json(input): Json<RefreshTokenInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<()> {
    Ok(AuthService::sign_out(input, &pool).await?)
}

pub(crate) async fn register(
    Json(input): Json<RegisterUserInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<(StatusCode, Json<LoginPayload>)> {
    validate_payload(&input)?;

    let user = AuthService::sign_up(input, &pool).await?;
    let token = jwt::sign(user.id)?;
    let refresh_token = AuthService::create_refresh_token(user.id, &pool).await?;

    Ok((
        StatusCode::CREATED,
        Json(LoginPayload {
            refresh_token,
            access_token: RefreshPayload {
                access_token: token,
                token_type: BEARER.to_string(),
            },
        }),
    ))
}

pub(crate) async fn update(
    user: User,
    Json(input): Json<UpdateUserInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<RefreshPayload>> {
    validate_payload(&input)?;

    let user = AuthService::update(user, input, &pool).await?;
    let token = jwt::sign(user.id)?;

    // TODO: Integrate with refresh tokens
    Ok(Json(RefreshPayload {
        access_token: token,
        token_type: BEARER.to_string(),
    }))
}

pub(crate) async fn token(
    Json(input): Json<RefreshTokenInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<RefreshPayload>> {
    let user = AuthService::refresh_access_token(input, &pool).await?;
    let token = jwt::sign(user.id)?;

    Ok(Json(RefreshPayload {
        access_token: token,
        token_type: BEARER.to_string(),
    }))
}
