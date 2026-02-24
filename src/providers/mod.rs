pub mod mock_provider;
pub mod email_provider;
pub mod sms_provider;
pub mod whatsapp_provider;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderSendRequest {
    pub channel: String,
    pub recipient: String,
    pub subject: Option<String>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderSendResponse {
    pub provider_message_id: String,
    pub status: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("provider error: {0}")]
    Message(String),
}

#[async_trait]
pub trait NotificationProvider {
    async fn send(&self, payload: ProviderSendRequest) -> Result<ProviderSendResponse, ProviderError>;
}
