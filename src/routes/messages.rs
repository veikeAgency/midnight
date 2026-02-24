use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    handlers::messages_handler,
    state::AppState,
    utils::errors::AppError,
};

pub async fn send_message(
    State(state): State<AppState>,
    Json(payload): Json<messages_handler::SendMessageRequest>,
) -> Result<Json<messages_handler::MessageActionResponse>, AppError> {
    Ok(Json(messages_handler::send_message(state, payload).await?))
}

pub async fn schedule_message(
    State(state): State<AppState>,
    Json(payload): Json<messages_handler::ScheduleMessageRequest>,
) -> Result<Json<messages_handler::MessageActionResponse>, AppError> {
    Ok(Json(messages_handler::schedule_message(state, payload).await?))
}

pub async fn message_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<messages_handler::MessageStatusResponse>, AppError> {
    Ok(Json(messages_handler::message_status(state, id).await?))
}
