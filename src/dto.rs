pub(crate) mod auth;
pub(crate) mod state;
pub(crate) mod task;

#[derive(Debug, Serialize)]
pub(crate) struct TokenPayload {
    pub(crate) access_token: String,
    pub(crate) token_type: String,
}
