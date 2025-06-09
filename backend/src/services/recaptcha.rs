use std::env;
use reqwest::Client;
use serde::Deserialize;
use crate::utils::errors::ApiError;

#[derive(Deserialize)]
struct RecaptchaResponse {
    success: bool,
    #[allow(dead_code)]
    score: Option<f32>, // si usas v3
    #[allow(dead_code)]
    challenge_ts: Option<String>,
    #[allow(dead_code)]
    hostname: Option<String>,
}

// 🎯 Valida token reCAPTCHA v2/v3 con la API de Google
pub async fn verify_recaptcha(token: &str) -> Result<bool, ApiError> {
    let secret_key = env::var("RECAPTCHA_SECRET_KEY")
        .map_err(|_| ApiError::MissingEnv("RECAPTCHA_SECRET_KEY".into()))?;

    let client = Client::new();
    let response = client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .form(&[
            ("secret", secret_key.as_str()),
            ("response", token),
        ])
        .send()
        .await
        .map_err(|e| ApiError::External(format!("Error de conexión: {}", e)))?;

    let data: RecaptchaResponse = response.json().await
        .map_err(|e| ApiError::External(format!("Respuesta inválida: {}", e)))?;

    Ok(data.success)
}
