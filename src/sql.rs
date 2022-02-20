use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    config::db::postgres::PgPool,
    error::Result,
    model::{
        auth::{CreateUserData, UpdateUserData},
        User,
    },
    schema::users,
};

mod health;
mod state;
mod task;

impl User {
    pub(crate) async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(users::table.find(id).first(&conn)?)
    }

    pub(crate) async fn find_by_email(email: &str, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(users::table.filter(users::email.eq(email)).first(&conn)?)
    }

    pub(crate) async fn find_by_name(name: &str, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(users::table.filter(users::name.eq(name)).first(&conn)?)
    }

    pub(crate) async fn create(data: CreateUserData, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(diesel::insert_into(users::table)
            .values(&data)
            .returning(users::all_columns)
            .get_result(&conn)?)
    }

    pub(crate) async fn update(id: Uuid, data: UpdateUserData, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(diesel::update(users::table.find(id))
            .set(&data)
            .get_result(&conn)?)
    }
}
