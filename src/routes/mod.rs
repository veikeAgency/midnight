pub mod health;
pub mod messages;
pub mod webhooks;
pub mod templates;
pub mod logs;

use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/ready", get(health::ready))
        .route("/messages/send", post(messages::send_message))
        .route("/messages/schedule", post(messages::schedule_message))
        .route("/messages/:id/status", get(messages::message_status))
        .route("/webhooks/provider", post(webhooks::provider_webhook))
        .route("/templates", post(templates::create_template).get(templates::list_templates))
        .route("/templates/:id", get(templates::get_template).patch(templates::update_template).delete(templates::delete_template))
        .route("/logs/messages", get(logs::list_message_logs))
        .route("/logs/messages/:id/events", get(logs::message_events))
}
