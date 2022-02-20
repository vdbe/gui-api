use crate::{
    config::db::postgres::PgPool,
    error::Result,
    model::health::{DbHealth, Health},
};
pub(crate) struct HealthService;

impl HealthService {
    pub(crate) async fn get(pool: &PgPool) -> Result<Health> {
        Ok(Health {
            db: DbHealth::check(pool).await,
        })
    }
}
