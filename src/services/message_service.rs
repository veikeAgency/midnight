use crate::utils::errors::AppError;

pub fn validate_send_payload(has_template: bool, has_body: bool) -> Result<(), AppError> {
    if !has_template && !has_body {
        return Err(AppError::BadRequest(
            "Provide either template_name or body".into(),
        ));
    }
    Ok(())
}
