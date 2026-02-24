use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: Uuid,
    pub provider: String,
    pub event_type: String,
    pub provider_message_id: Option<String>,
    pub payload: serde_json::Value,
    pub signature_valid: bool,
    pub processed: bool,
    pub received_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}
