use axum::{extract::State, Json};
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub success: bool,
    pub status: &'static str,
    pub service: &'static str,
    pub environment: String,
}

#[derive(Serialize)]
pub struct ReadyResponse {
    pub success: bool,
    pub db: &'static str,
    pub redis: &'static str,
}

pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        success: true,
        status: "ok",
        service: "midnight",
        environment: state.config.app_env,
    })
}

pub async fn ready(State(state): State<AppState>) -> Json<ReadyResponse> {
    let db_ok = sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_ok();

    let redis_ok = state.redis.get().await.is_ok();

    Json(ReadyResponse {
        success: db_ok && redis_ok,
        db: if db_ok { "ok" } else { "error" },
        redis: if redis_ok { "ok" } else { "error" },
    })
}
