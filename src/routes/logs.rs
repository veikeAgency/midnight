use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::{
    handlers::logs_handler,
    state::AppState,
    utils::errors::AppError,
};

pub async fn list_message_logs(
    State(state): State<AppState>,
    Query(query): Query<logs_handler::LogsQuery>,
) -> Result<Json<Vec<logs_handler::LogItemResponse>>, AppError> {
    Ok(Json(logs_handler::list_message_logs(state, query).await?))
}

pub async fn message_events(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<logs_handler::MessageEventResponse>>, AppError> {
    Ok(Json(logs_handler::message_events(state, id).await?))
}
