use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
};
use headers::{authorization::Bearer, Authorization};

use crate::{
    config::db::postgres::PgPool,
    error::{ApiError, Error},
    model::User,
    util::{jwt, jwt::Claims},
};

#[async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = ApiError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(Error::from)?;
        let Extension(pool) = Extension::<PgPool>::from_request(req)
            .await
            .map_err(Error::from)?;
        let claims = jwt::verify(bearer.token())?;
        Ok(User::find_by_id(claims.sub, &pool).await?)
    }
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = ApiError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(Error::from)?;
        let claims = jwt::verify(bearer.token())?;
        Ok(claims)
    }
}
