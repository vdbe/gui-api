use diesel::{pg::PgConnection, r2d2};

use crate::config::db::DbPool;

pub type PgPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

impl DbPool for PgPool {
    fn retrieve() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);

        let pg_pool: PgPool = r2d2::Pool::builder()
            .build(manager)
            .expect("DB connection was failed");

        pg_pool
    }
}
