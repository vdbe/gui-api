use axum::{
    async_trait,
    extract::{Extension, FromRequest, Path, RequestParts, TypedHeader},
};
use headers::{authorization::Bearer, Authorization};

use crate::{
    config::db::postgres::PgPool,
    dto::IdentifierPath,
    error::{ApiError, Error},
    model::{task::Task, User},
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
impl<B> FromRequest<B> for Task
where
    B: Send,
{
    type Rejection = ApiError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Path(identifier): Path<IdentifierPath> =
            Path::from_request(req).await.map_err(Error::from)?;

        let Extension(pool) = Extension::<PgPool>::from_request(req)
            .await
            .map_err(Error::from)?;

        Ok(match identifier {
            IdentifierPath::Integer(nr) => Task::find_by_nr(nr, &pool).await,
            IdentifierPath::Text(_) => return Err(Error::InvalidIdentifier.into()),
        }?)
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
