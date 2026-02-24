use crate::utils::errors::AppError;

// TODO: Build axum middleware that:
// 1) reads Authorization Bearer or x-api-key
// 2) hashes incoming key
// 3) finds active api_keys row
// 4) inserts tenant context into request extensions
pub async fn require_api_key_stub() -> Result<(), AppError> {
    Ok(())
}
