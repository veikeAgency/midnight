use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    services::message_service,
    state::AppState,
    utils::errors::AppError,
};

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub channel: String,
    pub provider: Option<String>,
    pub recipient: String,
    pub template_name: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleMessageRequest {
    pub channel: String,
    pub provider: Option<String>,
    pub recipient: String,
    pub template_name: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub idempotency_key: Option<String>,
    pub scheduled_at: DateTime<Utc>,
    pub max_attempts: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct MessageActionResponse {
    pub success: bool,
    pub message_id: Uuid,
    pub status: String,
    pub scheduled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct MessageStatusResponse {
    pub id: Uuid,
    pub status: String,
    pub channel: String,
    pub recipient: String,
    pub provider: String,
    pub attempt_count: i32,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub provider_message_id: Option<String>,
}

pub async fn send_message(
    state: AppState,
    payload: SendMessageRequest,
) -> Result<MessageActionResponse, AppError> {
    message_service::validate_send_payload(payload.template_name.is_some(), payload.body.is_some())?;

    let id = Uuid::new_v4();
    let provider = payload.provider.unwrap_or_else(|| state.config.default_provider.clone());

    // TODO:
    // - resolve template if template_name exists
    // - insert message row
    // - push queue job to redis
    // - append message_logs row
    let _ = provider;

    Ok(MessageActionResponse {
        success: true,
        message_id: id,
        status: "queued".into(),
        scheduled_at: None,
    })
}

pub async fn schedule_message(
    state: AppState,
    payload: ScheduleMessageRequest,
) -> Result<MessageActionResponse, AppError> {
    message_service::validate_send_payload(payload.template_name.is_some(), payload.body.is_some())?;

    let id = Uuid::new_v4();
    let provider = payload.provider.unwrap_or_else(|| state.config.default_provider.clone());

    let _ = provider;
    let _ = payload.max_attempts;

    // TODO:
    // - insert scheduled message row
    // - scheduler worker picks due jobs
    // - append message_logs row
    Ok(MessageActionResponse {
        success: true,
        message_id: id,
        status: "queued".into(),
        scheduled_at: Some(payload.scheduled_at),
    })
}

pub async fn message_status(
    _state: AppState,
    id: Uuid,
) -> Result<MessageStatusResponse, AppError> {
    // TODO: query messages table
    Ok(MessageStatusResponse {
        id,
        status: "queued".into(),
        channel: "sms".into(),
        recipient: "+256700000000".into(),
        provider: "mock".into(),
        attempt_count: 0,
        scheduled_at: None,
        sent_at: None,
        delivered_at: None,
        last_error: None,
        provider_message_id: None,
    })
}