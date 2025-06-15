use actix_web::{post, web, HttpResponse, Scope};
use serde::Deserialize;
use sqlx::PgPool;
use crate::services::recaptcha::verify_recaptcha;
use crate::handlers::login;
use argon2::{Argon2, PasswordHasher};
use password_hash::SaltString;
use rand_core::OsRng;

#[derive(Deserialize)]
pub struct RegisterInput {
    pub email: String,
    pub password: String,
    pub token: String,
}

#[post("/register")]
async fn register_user(
    db: web::Data<PgPool>,
    form: web::Json<RegisterInput>,
) -> HttpResponse {
    match verify_recaptcha(&form.token).await {
        Ok(true) => {}
        Ok(false) => return HttpResponse::BadRequest().body("❌ reCAPTCHA inválido"),
        Err(e) => {
            eprintln!("Error reCAPTCHA: {:?}", e);
            return HttpResponse::InternalServerError().body("❌ Error en reCAPTCHA");
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = match argon2.hash_password(form.password.as_bytes(), &salt) {
        Ok(pwd_hash) => pwd_hash.to_string(),
        Err(_) => return HttpResponse::InternalServerError().body("❌ Fallo al encriptar"),
    };

    let result = sqlx::query("INSERT INTO users (email, password) VALUES ($1, $2)")
        .bind(&form.email)
        .bind(&hash)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("✅ Usuario registrado correctamente"),
        Err(e) => {
            eprintln!("Error BD: {:?}", e);
            HttpResponse::InternalServerError().body("❌ Error al registrar")
        }
    }
}

pub fn routes() -> Scope {
    web::scope("/auth") // 
        .service(register_user)
        .service(login::login_user)
}
