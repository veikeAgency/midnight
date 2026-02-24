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

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{config::AppConfig, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::from_env();

    tracing_subscriber::registry()
        .with(EnvFilter::new(config.log_level.clone()))
        .with(tracing_subscriber::fmt::layer())
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
    tracing::info!("midnight listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
