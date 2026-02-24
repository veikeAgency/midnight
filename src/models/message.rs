use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub channel: String,
    pub provider: String,
    pub recipient: String,
    pub template_id: Option<Uuid>,
    pub subject: Option<String>,
    pub body: String,
    pub variables: Option<serde_json::Value>,
    pub status: String,
    pub provider_message_id: Option<String>,
    pub provider_response: Option<serde_json::Value>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub queued_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub attempt_count: i32,
    pub max_attempts: i32,
    pub next_retry_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub idempotency_key: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
