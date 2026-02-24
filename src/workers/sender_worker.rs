// TODO: Implement sender worker loop:
// - read from Redis queue
// - fetch message from DB
// - call provider_service
// - update status + logs
pub async fn run_sender_worker() -> anyhow::Result<()> {
    Ok(())
}
