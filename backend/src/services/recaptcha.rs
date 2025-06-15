use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct RecaptchaResponse {
    success: bool,
    #[allow(dead_code)]
    challenge_ts: Option<String>,
    #[allow(dead_code)]
    hostname: Option<String>,
}

pub async fn verify_recaptcha(token: &str) -> Result<bool, reqwest::Error> {
    let secret = env::var("SECRET_KEY").expect("Falta SECRET_KEY en .env");

    let client = Client::new();
    let res = client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .form(&[("secret", secret.as_str()), ("response", token)])
        .send()
        .await?
        .json::<RecaptchaResponse>()
        .await?;

    Ok(res.success)
}
