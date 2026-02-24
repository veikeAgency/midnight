use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{state::AppState, utils::errors::AppError};

#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub channel: String,
    pub subject: Option<String>,
    pub body: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct ListTemplatesQuery {
    pub channel: Option<String>,
    pub active_only: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTemplateRequest {
    pub subject: Option<String>,
    pub body: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct TemplateResponse {
    pub id: Uuid,
    pub name: String,
    pub channel: String,
    pub subject: Option<String>,
    pub body: String,
    pub is_active: bool,
    pub version: i32,
}

#[derive(Debug, Serialize)]
pub struct DeleteTemplateResponse {
    pub success: bool,
    pub id: Uuid,
}

pub async fn create_template(
    _state: AppState,
    payload: CreateTemplateRequest,
) -> Result<TemplateResponse, AppError> {
    Ok(TemplateResponse {
        id: Uuid::new_v4(),
        name: payload.name,
        channel: payload.channel,
        subject: payload.subject,
        body: payload.body,
        is_active: true,
        version: 1,
    })
}

pub async fn list_templates(
    _state: AppState,
    _query: ListTemplatesQuery,
) -> Result<Vec<TemplateResponse>, AppError> {
    Ok(vec![])
}

pub async fn get_template(
    _state: AppState,
    id: Uuid,
) -> Result<TemplateResponse, AppError> {
    Ok(TemplateResponse {
        id,
        name: "appointment_reminder".into(),
        channel: "sms".into(),
        subject: None,
        body: "Hello {{name}}, your appointment is {{time}}".into(),
        is_active: true,
        version: 1,
    })
}

pub async fn update_template(
    _state: AppState,
    id: Uuid,
    payload: UpdateTemplateRequest,
) -> Result<TemplateResponse, AppError> {
    Ok(TemplateResponse {
        id,
        name: "updated_template".into(),
        channel: "sms".into(),
        subject: payload.subject,
        body: payload.body.unwrap_or_else(|| "Updated body".into()),
        is_active: payload.is_active.unwrap_or(true),
        version: 1,
    })
}

pub async fn delete_template(
    _state: AppState,
    id: Uuid,
) -> Result<DeleteTemplateResponse, AppError> {
    Ok(DeleteTemplateResponse { success: true, id })
}