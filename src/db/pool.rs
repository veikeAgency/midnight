use deadpool_redis::{Config as RedisConfig, Pool, Runtime};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}

pub fn redis_pool(redis_url: &str) -> anyhow::Result<Pool> {
    let cfg = RedisConfig::from_url(redis_url);
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
    Ok(pool)
}
