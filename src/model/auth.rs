use std::time::SystemTime;

use uuid::Uuid;

use crate::schema::{refreshtokens, users};

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub(crate) struct CreateUserData {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) created_at: SystemTime,
    pub(crate) updated_at: SystemTime,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub(crate) struct UpdateUserData {
    pub(crate) name: Option<String>,
    pub(crate) email: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) updated_at: Option<SystemTime>,
}

#[derive(Debug, Insertable)]
#[table_name = "refreshtokens"]
pub(crate) struct CreateRefreshTokenData {
    pub(crate) user_id: Uuid,
    pub(crate) expiry_date: SystemTime,
}

#[derive(Debug, Serialize, Queryable)]
pub(crate) struct RefreshToken {
    #[serde(skip_serializing)]
    pub(crate) id: Uuid,
    #[serde(rename = "refresh_token")]
    pub(crate) token: Uuid,
    #[serde(skip_serializing)]
    pub(crate) user_id: Uuid,
    #[serde(skip_serializing)]
    pub(crate) expiry_date: SystemTime,
}
