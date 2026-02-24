use axum::{body::Bytes, extract::State, Json};

use crate::{handlers::webhooks_handler, state::AppState};

pub async fn provider_webhook(
    State(state): State<AppState>,
    body: Bytes,
) -> Json<webhooks_handler::WebhookResponse> {
    Json(webhooks_handler::provider_webhook(state, body).await)
}
