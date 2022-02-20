use serde::de::{Deserialize, Deserializer};
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct CreateInput {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) progress: i32,
}

#[derive(Debug)]
pub(crate) enum IdentifierInput {
    Progress(i32),
    Name(String),
    Id(Uuid),
}

impl<'de> Deserialize<'de> for IdentifierInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input: String = Deserialize::deserialize(deserializer)?;

        let identifier: IdentifierInput = match input.parse::<i32>() {
            Ok(num) => Self::Progress(num),
            Err(_) => Self::Name(input),
        };

        Ok(identifier)
    }
}
