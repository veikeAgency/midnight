use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;

use crate::{
    handlers::templates_handler,
    state::AppState,
    utils::errors::AppError,
};

pub async fn create_template(
    State(state): State<AppState>,
    Json(payload): Json<templates_handler::CreateTemplateRequest>,
) -> Result<Json<templates_handler::TemplateResponse>, AppError> {
    Ok(Json(templates_handler::create_template(state, payload).await?))
}

pub async fn list_templates(
    State(state): State<AppState>,
    Query(query): Query<templates_handler::ListTemplatesQuery>,
) -> Result<Json<Vec<templates_handler::TemplateResponse>>, AppError> {
    Ok(Json(templates_handler::list_templates(state, query).await?))
}

pub async fn get_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<templates_handler::TemplateResponse>, AppError> {
    Ok(Json(templates_handler::get_template(state, id).await?))
}

pub async fn update_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<templates_handler::UpdateTemplateRequest>,
) -> Result<Json<templates_handler::TemplateResponse>, AppError> {
    Ok(Json(templates_handler::update_template(state, id, payload).await?))
}

pub async fn delete_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<templates_handler::DeleteTemplateResponse>, AppError> {
    Ok(Json(templates_handler::delete_template(state, id).await?))
}
