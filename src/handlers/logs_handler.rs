use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{state::AppState, utils::errors::AppError};

#[derive(Debug, Deserialize, Default)]
pub struct LogsQuery {
    pub status: Option<String>,
    pub channel: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct LogItemResponse {
    pub message_id: Uuid,
    pub channel: String,
    pub recipient: String,
    pub status: String,
    pub attempts: i32,
    pub created_at: DateTime<Utc>,
    pub last_error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MessageEventResponse {
    pub event: String,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

pub async fn list_message_logs(
    _state: AppState,
    _query: LogsQuery,
) -> Result<Vec<LogItemResponse>, AppError> {
    Ok(vec![])
}

pub async fn message_events(
    _state: AppState,
    _id: Uuid,
) -> Result<Vec<MessageEventResponse>, AppError> {
    Ok(vec![])
}
