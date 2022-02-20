use crate::{config::db::postgres::PgPool, model::health::DbHealth};

impl DbHealth {
    pub(crate) async fn check(pool: &PgPool) -> Self {
        match pool.get() {
            Ok(_) => Self::Available,
            Err(_) => Self::Unavailable,
        }
    }
}
