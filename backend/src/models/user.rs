use serde::Deserialize;
use validator::Validate;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub recaptcha_token: String,
}
