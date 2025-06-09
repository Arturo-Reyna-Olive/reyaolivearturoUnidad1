use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use reqwest;

#[derive(Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    pub token: String, // <-- reCAPTCHA token
}

#[derive(Deserialize)]
struct RecaptchaResponse {
    success: bool,
    #[serde(rename = "error-codes")]
    error_codes: Option<Vec<String>>,
}

pub async fn register_user(
    db: web::Data<PgPool>,
    form: web::Json<RegisterForm>,
) -> HttpResponse {
    let secret_key = std::env::var("RECAPTCHA_SECRET_KEY").expect("🔐 Falta RECAPTCHA_SECRET_KEY");

    let client = reqwest::Client::new();
    let recaptcha_res = client.post("https://www.google.com/recaptcha/api/siteverify")
        .form(&[
            ("secret", secret_key.as_str()),
            ("response", &form.token),
        ])
        .send()
        .await;

    match recaptcha_res {
        Ok(resp) => {
            let body: RecaptchaResponse = match resp.json().await {
                Ok(data) => data,
                Err(_) => return HttpResponse::InternalServerError().body("❌ Error al procesar CAPTCHA"),
            };

            if !body.success {
                return HttpResponse::Unauthorized().body("🚫 reCAPTCHA inválido");
            }
        }
        Err(_) => return HttpResponse::InternalServerError().body("❌ No se pudo validar reCAPTCHA"),
    }

    let result = sqlx::query!(
        "INSERT INTO users (email, password) VALUES ($1, $2)",
        form.email,
        form.password
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("✅ Registro con CAPTCHA exitoso"),
        Err(e) => {
            eprintln!("❌ Error BD: {:?}", e);
            HttpResponse::InternalServerError().body("❌ Error interno al registrar")
        }
    }
}
