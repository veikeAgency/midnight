use crate::{
    providers::{mock_provider::MockProvider, NotificationProvider, ProviderSendRequest, ProviderSendResponse},
    utils::errors::AppError,
};

pub async fn send_with_provider(
    provider: &str,
    payload: ProviderSendRequest,
) -> Result<ProviderSendResponse, AppError> {
    match provider {
        "mock" => {
            let p = MockProvider;
            p.send(payload)
                .await
                .map_err(|_| AppError::Internal)
        }
        _ => Err(AppError::BadRequest(format!("Unsupported provider: {}", provider))),
    }
}
