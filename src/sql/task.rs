use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    config::db::postgres::PgPool,
    dto::state::IdentifierInput,
    error::Result,
    model::task::{CreateTaskData, Task},
    schema::tasks,
};

impl Task {
    pub(crate) async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(tasks::table.find(id).first(&conn)?)
    }

    pub(crate) async fn find_by_name(title: &str, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(tasks::table.filter(tasks::title.eq(title)).first(&conn)?)
    }

    pub(crate) async fn create(data: CreateTaskData, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(diesel::insert_into(tasks::table)
            .values(&data)
            .returning(tasks::all_columns)
            .get_result(&conn)?)
    }
}
