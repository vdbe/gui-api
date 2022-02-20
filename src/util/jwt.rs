use std::time::Duration;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{config::env::JWT_SECRET, error::Result};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub(crate) fn new(id: Uuid) -> Self {
        let iat = OffsetDateTime::now_utc();
        let exp = iat + Duration::from_secs(24 * 60 * 60);

        Self {
            sub: id,
            iat: iat.unix_timestamp(),
            exp: exp.unix_timestamp(),
        }
    }
}

pub(crate) fn sign(id: Uuid) -> Result<String> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id),
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )?)
}

pub(crate) fn verify(token: &str) -> Result<Claims> {
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)?)
}
