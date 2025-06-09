use actix_web::{HttpResponse, ResponseError};
use serde_json::json;

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(json!({ "error": msg }))
            }
            ApiError::External(msg) => {
                HttpResponse::BadGateway().json(json!({ "error": msg }))
            }
            ApiError::MissingEnv(var) => {
                HttpResponse::InternalServerError().json(json!({ "error": format!("Falta var de entorno: {}", var) }))
            }
            ApiError::RecaptchaFailed => {
                HttpResponse::Unauthorized().json(json!({ "error": "reCAPTCHA inválido" }))
            }
        }
    }
}


