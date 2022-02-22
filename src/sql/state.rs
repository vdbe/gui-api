use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    config::db::postgres::PgPool,
    dto::IdentifierInput,
    error::Result,
    model::state::{CreateStateData, State, UpdateStateData},
    schema::states,
};

impl State {
    pub(crate) async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(states::table.find(id).first(&conn)?)
    }

    pub(crate) async fn find_by_name(name: &str, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(states::table.filter(states::name.eq(name)).first(&conn)?)
    }

    pub(crate) async fn find_by_progress(progress: i32, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(states::table
            .filter(states::progress.eq(progress))
            .first(&conn)?)
    }

    pub(crate) async fn find_id(identifier: IdentifierInput, pool: &PgPool) -> Result<Uuid> {
        if let IdentifierInput::Id(id) = identifier {
            return Ok(id);
        };

        let conn = pool.get()?;

        let id = match identifier {
            IdentifierInput::Integer(p) => states::table
                .select(states::id)
                .filter(states::progress.eq(p))
                .first(&conn)?,
            IdentifierInput::Text(n) => states::table
                .select(states::id)
                .filter(states::name.eq(n))
                .first(&conn)?,
            IdentifierInput::Id(_) => unreachable!(),
        };

        Ok(id)
    }

    pub(crate) async fn get_all(pool: &PgPool) -> Result<Vec<Self>> {
        let conn = pool.get()?;

        Ok(states::table.load(&conn)?)
    }

    pub(crate) async fn create(data: CreateStateData, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(diesel::insert_into(states::table)
            .values(&data)
            .returning(states::all_columns)
            .get_result(&conn)?)
    }

    pub(crate) async fn update(id: Uuid, data: UpdateStateData, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(dbg!(diesel::update(states::table.find(id))
            .set(&data)
            .get_result(&conn))?)
    }
}
