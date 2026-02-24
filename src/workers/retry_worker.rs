// TODO: Implement retry worker loop:
// - poll messages where next_retry_at <= now and status=queued
// - enqueue to notify:send
pub async fn run_retry_worker() -> anyhow::Result<()> {
    Ok(())
}
