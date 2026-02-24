use axum::body::Bytes;
use serde::Serialize;

use crate::{state::AppState, utils::signatures};

#[derive(Debug, Serialize)]
pub struct WebhookResponse {
    pub received: bool,
}

pub async fn provider_webhook(_state: AppState, body: Bytes) -> WebhookResponse {
    // TODO:
    // - verify signature header (provider-specific)
    // - persist webhook_events
    // - map provider_message_id -> messages
    // - update statuses + message_logs
    let _ = signatures::noop_signature_check();
    let _raw = body;

    WebhookResponse { received: true }
}
