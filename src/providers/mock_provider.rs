use async_trait::async_trait;
use rand::{distr::Alphanumeric, Rng};

use crate::providers::{NotificationProvider, ProviderError, ProviderSendRequest, ProviderSendResponse};

pub struct MockProvider;

#[async_trait]
impl NotificationProvider for MockProvider {
    async fn send(&self, _payload: ProviderSendRequest) -> Result<ProviderSendResponse, ProviderError> {
        let id: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        Ok(ProviderSendResponse {
            provider_message_id: format!("mock_{}", id),
            status: "sent".into(),
        })
    }
}
