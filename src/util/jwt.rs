use std::time::Duration;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Serialize};
use time::OffsetDateTime;

use crate::{config::env::JWT_SECRET, error::Result};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Claims<T> {
    pub sub: T,
    pub exp: i64,
    pub iat: i64,
}

impl<T> Claims<T> {
    pub(crate) fn new(claim: T) -> Self {
        let iat = OffsetDateTime::now_utc();
        let exp = iat + Duration::from_secs(5 * 60);

        Self {
            sub: claim,
            iat: iat.unix_timestamp(),
            exp: exp.unix_timestamp(),
        }
    }
}

pub(crate) fn sign<T>(claim: T) -> Result<String>
where
    T: Serialize,
{
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(claim),
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )?)
}

pub(crate) fn verify<T>(token: &str) -> Result<T>
where T:  DeserializeOwned
{
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)?)
}
