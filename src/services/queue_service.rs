use deadpool_redis::Pool;
use redis::AsyncCommands;

pub async fn enqueue_json(pool: &Pool, queue: &str, payload: &str) -> anyhow::Result<()> {
    let mut conn = pool.get().await?;
    let _: i64 = conn.rpush(queue, payload).await?;
    Ok(())
}
