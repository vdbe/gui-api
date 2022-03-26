use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    config::db::postgres::PgPool,
    error::Result,
    model::auth::{CreateRefreshTokenData, RefreshToken},
    schema::refreshtokens,
};

impl RefreshToken {
    pub(crate) async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(refreshtokens::table.find(id).first(&conn)?)
    }

    pub(crate) async fn find_by_token(token: Uuid, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(refreshtokens::table
            .filter(refreshtokens::token.eq(token))
            .first(&conn)?)
    }

    pub(crate) async fn drop_by_id(id: Uuid, pool: &PgPool) -> Result<()> {
        let conn = pool.get()?;

        let _rows_changed = diesel::delete(refreshtokens::table.find(id)).execute(&conn)?;

        Ok(())
    }

    pub(crate) async fn drop_by_token(token: Uuid, pool: &PgPool) -> Result<()> {
        let conn = pool.get()?;

        let _rows_changed =
            diesel::delete(refreshtokens::table.filter(refreshtokens::token.eq(token)))
                .execute(&conn)?;

        Ok(())
    }

    pub(crate) async fn create(data: CreateRefreshTokenData, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(diesel::insert_into(refreshtokens::table)
            .values(&data)
            .returning(refreshtokens::all_columns)
            .get_result(&conn)?)
    }
}
