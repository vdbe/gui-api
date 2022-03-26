use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct LoginUserInput {
    #[validate(email)]
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct RegisterUserInput {
    #[validate(length(min = 4, max = 10))]
    pub(crate) name: String,
    #[validate(email)]
    pub(crate) email: String,
    #[validate(length(min = 6))]
    pub(crate) password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct UpdateUserInput {
    #[validate(length(min = 6))]
    pub(crate) password: String,
    #[validate(length(min = 4, max = 10))]
    pub(crate) name: Option<String>,
    #[validate(email)]
    pub(crate) email: Option<String>,
    #[validate(length(min = 6))]
    pub(crate) new_password: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub(crate) struct RefreshTokenInput {
    #[serde(rename = "refresh_token")]
    pub(crate) token: Uuid,
}
