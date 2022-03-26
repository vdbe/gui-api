use serde::de::{self, Deserialize, Deserializer};
use uuid::Uuid;

use crate::model::auth::RefreshToken;

pub(crate) mod auth;
pub(crate) mod state;
pub(crate) mod task;

#[derive(Debug, Serialize)]
pub(crate) struct RefreshPayload {
    pub(crate) access_token: String,
    pub(crate) token_type: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct LoginPayload {
    #[serde(flatten)]
    pub(crate) refresh_token: RefreshToken,
    #[serde(flatten)]
    pub(crate) access_token: RefreshPayload,
}

#[derive(Debug)]
pub(crate) enum IdentifierInput {
    Integer(i32),
    Text(String),
    Id(Uuid),
}

#[derive(Debug)]
pub(crate) enum IdentifierPath {
    Integer(i32),
    Text(String),
}

impl From<IdentifierPath> for IdentifierInput {
    fn from(identifier: IdentifierPath) -> Self {
        match identifier {
            IdentifierPath::Integer(integer) => Self::Integer(integer),
            IdentifierPath::Text(text) => Self::Text(text),
        }
    }
}

impl<'de> Deserialize<'de> for IdentifierInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input: serde_json::value::Value = Deserialize::deserialize(deserializer)?;

        let identifier = match input {
            serde_json::Value::Number(number) => {
                let x = match i32::try_from(
                    number
                        .as_i64()
                        .ok_or_else(|| de::Error::custom("progress not a i32"))?,
                ) {
                    Ok(n) => n,
                    Err(_) => return Err(de::Error::custom("progress not an i32")),
                };

                Self::Integer(x)
            }
            serde_json::Value::String(name) => Self::Text(name),
            _ => return Err(de::Error::custom("Invalid type")),
        };

        Ok(identifier)
    }
}

impl<'de> Deserialize<'de> for IdentifierPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input: String = Deserialize::deserialize(deserializer)?;

        let identifier: Self = match input.parse::<i32>() {
            Ok(num) => Self::Integer(num),
            Err(_) => Self::Text(input),
        };

        Ok(identifier)
    }
}
