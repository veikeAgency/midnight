mod app;
mod config;
mod state;
mod routes;
mod handlers;
mod middleware;
mod models;
mod services;
mod providers;
mod workers;
mod db;
mod utils;

use std::net::SocketAddr;

use tracing_subscriber::EnvFilter;

use crate::{config::AppConfig, state::AppState};

fn main() -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(run())
}

async fn run() -> anyhow::Result<()> {
    let config = AppConfig::from_env();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(config.log_level.clone()))
        .init();

    let db = db::pool::connect(&config.database_url).await?;
    let redis = db::pool::redis_pool(&config.redis_url)?;

    let state = AppState {
        config: config.clone(),
        db,
        redis,
    };

    let app = app::build_router(state);

    let addr: SocketAddr = config.addr().parse()?;
    println!("midnight listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
