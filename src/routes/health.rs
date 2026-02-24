use axum::{extract::State, Json};
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    success: bool,
    status: &'static str,
    service: &'static str,
    environment: String,
}

pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        success: true,
        status: "ok",
        service: "midnight",
        environment: state.config.app_env.clone(),
    })
}

#[derive(Serialize)]
pub struct ReadyResponse {
    success: bool,
    db: &'static str,
}

pub async fn ready(State(state): State<AppState>) -> Json<ReadyResponse> {
    let db_ok = sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_ok();

    Json(ReadyResponse {
        success: db_ok,
        db: if db_ok { "ok" } else { "error" },
    })
}
