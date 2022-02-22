use diesel::{
    deserialize::QueryableByName, dsl::sql, pg::Pg, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use uuid::Uuid;

use crate::{
    config::db::postgres::PgPool,
    dto::task::TaskOutput,
    error::Result,
    model::task::{CreateTaskData, Task, UpdateTaskData},
    //schema::{tasks, states},
    schema::*,
};

impl Task {
    pub(crate) async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(tasks::table.find(id).first(&conn)?)
    }

    pub(crate) async fn find_by_nr(nr: i32, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(tasks::table.filter(tasks::nr.eq(nr)).first(&conn)?)
    }

    pub(crate) async fn get_all(pool: &PgPool) -> Result<Vec<Self>> {
        let conn = pool.get()?;

        Ok(tasks::table.load(&conn)?)
    }

    pub(crate) async fn create(data: CreateTaskData, pool: &PgPool) -> Result<Task> {
        let conn = pool.get()?;

        Ok(diesel::insert_into(tasks::table)
            .values(&data)
            .returning(tasks::all_columns)
            .get_result(&conn)?)
    }

    pub(crate) async fn update(id: Uuid, data: UpdateTaskData, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(diesel::update(tasks::table.find(id))
            .set(&data)
            .get_result(&conn)?)
    }
}

impl TaskOutput {
    pub(crate) async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(tasks::table
            .find(id)
            .select((
                tasks::nr,
                sql("(select s.progress from states as s where s.id = state)"),
                sql("(select u.email from users as u where id = created_by)"),
                sql("(select u.email from users as u where id = taken_by)"),
                tasks::created_at,
                tasks::taken_at,
                tasks::completed_at,
                tasks::title,
                tasks::description,
            ))
            .first(&conn)?)
    }

    pub(crate) async fn find_by_nr(nr: i32, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(tasks::table
            .filter(tasks::nr.eq(nr))
            .select((
                tasks::nr,
                sql("(select s.progress from states as s where s.id = state)"),
                sql("(select u.email from users as u where id = created_by)"),
                sql("(select u.email from users as u where id = taken_by)"),
                tasks::created_at,
                tasks::taken_at,
                tasks::completed_at,
                tasks::title,
                tasks::description,
            ))
            .first(&conn)?)
    }

    pub(crate) async fn get_all(pool: &PgPool) -> Result<Vec<Self>> {
        let conn = pool.get()?;

        Ok(tasks::table
            .select((
                tasks::nr,
                sql("(select s.progress from states as s where s.id = state)"),
                sql("(select u.email from users as u where id = created_by)"),
                sql("(select u.email from users as u where id = taken_by)"),
                tasks::created_at,
                tasks::taken_at,
                tasks::completed_at,
                tasks::title,
                tasks::description,
            ))
            .load(&conn)?)
    }

    pub(crate) async fn update(id: Uuid, data: UpdateTaskData, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        let data: TaskOutput = diesel::update(tasks::table.find(id))
            .set(&data)
            .returning((
                tasks::nr,
                sql("(select s.progress from states as s where s.id = state)"),
                sql("(select u.email from users as u where id = created_by)"),
                sql("(select u.email from users as u where id = taken_by)"),
                tasks::created_at,
                tasks::taken_at,
                tasks::completed_at,
                tasks::title,
                tasks::description,
            ))
            .get_result(&conn)?;

        Ok(data)
    }
}

impl QueryableByName<Pg> for TaskOutput {
    fn build<R: diesel::row::NamedRow<Pg>>(row: &R) -> diesel::deserialize::Result<Self> {
        Ok(TaskOutput {
            nr: row.get("nr")?,
            progress: row.get("progress")?,
            created_by: row.get("created_by")?,
            taken_by: row.get("taken_by")?,
            created_at: row.get("created_at")?,
            taken_at: row.get("taken_at")?,
            completed_at: row.get("completed_at")?,
            title: row.get("title")?,
            description: row.get("description")?,
        })
    }
}
