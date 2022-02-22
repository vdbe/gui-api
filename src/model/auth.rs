use std::time::SystemTime;

use crate::schema::users;

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
