use diesel::{
    deserialize::QueryableByName, dsl::sql, pg::Pg, ExpressionMethods, NullableExpressionMethods,
    PgTextExpressionMethods, QueryDsl, RunQueryDsl,
};
use uuid::Uuid;

use crate::{
    config::db::postgres::PgPool,
    dto::task::TaskOutput,
    error::Result,
    model::{
        state::State,
        task::{CreateTaskData, SearchTaskData, Task, UpdateTaskData},
        User,
    },
    schema::{states, tasks, users},
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

    pub(crate) async fn create(data: CreateTaskData, pool: &PgPool) -> Result<Self> {
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
    #[inline(always)]
    fn get_select() -> (
        tasks::nr,
        diesel::expression::sql_literal::SqlLiteral<diesel::sql_types::Integer>,
        diesel::expression::sql_literal::SqlLiteral<diesel::sql_types::Text>,
        diesel::expression::sql_literal::SqlLiteral<
            diesel::sql_types::Nullable<diesel::sql_types::Text>,
        >,
        tasks::created_at,
        tasks::taken_at,
        tasks::completed_at,
        tasks::title,
        tasks::description,
    ) {
        // TODO: Find a beter way for this
        (
            tasks::nr,
            sql("(select s.progress from states as s where s.id = state)"),
            sql("(select u.email from users as u where id = created_by)"),
            sql("(select u.email from users as u where id = taken_by)"),
            tasks::created_at,
            tasks::taken_at,
            tasks::completed_at,
            tasks::title,
            tasks::description,
        )
    }

    pub(crate) async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(tasks::table
            .find(id)
            .select(Self::get_select())
            .first(&conn)?)
    }

    pub(crate) async fn find_by_nr(nr: i32, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(tasks::table
            .filter(tasks::nr.eq(nr))
            .select(Self::get_select())
            .first(&conn)?)
    }

    pub(crate) async fn get_all(pool: &PgPool) -> Result<Vec<Self>> {
        let conn = pool.get()?;

        Ok(tasks::table.select(Self::get_select()).load(&conn)?)
    }

    pub(crate) async fn search(data: SearchTaskData, pool: &PgPool) -> Result<Vec<Self>> {
        let mut query = tasks::table.into_boxed();

        if let Some(progress) = data.progress {
            let state_id = states::table
                .filter(states::progress.eq(progress))
                .select(states::id)
                .single_value();
            query = query.filter(tasks::state.nullable().eq(state_id));
        };

        if let Some(created_by) = data.created_by {
            let created_by_id = users::table
                .filter(users::email.eq(created_by))
                .select(users::id)
                .single_value();
            query = query.filter(tasks::created_by.nullable().eq(created_by_id));
        };

        if let Some(taken_by) = data.taken_by {
            let taken_by_id = users::table
                .filter(users::email.eq(taken_by))
                .select(users::id)
                .single_value();
            query = query.filter(tasks::taken_by.nullable().eq(taken_by_id));
        };

        if let Some(title) = data.title {
            query = query.filter(tasks::title.ilike(title));
        };

        if let Some(description) = data.description {
            query = query.filter(tasks::description.ilike(description));
        };

        let conn = pool.get()?;
        Ok(query.select(Self::get_select()).load(&conn)?)
    }

    pub(crate) async fn create(data: CreateTaskData, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(diesel::insert_into(tasks::table)
            .values(&data)
            .returning(Self::get_select())
            .get_result(&conn)?)
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
