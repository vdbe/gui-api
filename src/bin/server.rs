use std::{
    env,
    net::{IpAddr, SocketAddr},
};

use clap::Parser;
use tracing_subscriber::EnvFilter;

use gui_api::{
    app,
    config::db::{postgres::PgPool, DbPool},
};

#[derive(Debug, Parser)]
pub struct Config {
    #[clap(short, long, default_value = "127.0.0.1", env)]
    pub host: IpAddr,
    #[clap(short, long, default_value = "3000", env)]
    pub port: u16,
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let args = Config::parse();

    _ = env::var("JWT_SECRET").expect("environment variable `JWT_SECRET` must be set");

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let pg_pool = PgPool::retrieve();

    // Run it
    let addr = SocketAddr::from((args.host, args.port));
    tracing::info!("listening on {addr}");
    let server = axum::Server::bind(&addr).serve(app(pg_pool.clone()).into_make_service());

    if let Err(err) = server.await {
        tracing::error!("server error: {:?}", err);
    }
}
