use actix_session::Session;
use actix_web::{post, get, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use argon2::{Argon2, PasswordHash, PasswordVerifier};

#[derive(Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login_user(
    db: web::Data<PgPool>,
    form: web::Json<LoginInput>,
    session: Session,
) -> HttpResponse {
    let result = sqlx::query!(
        "SELECT id, email, password FROM users WHERE email = $1",
        form.email
    )
    .fetch_optional(db.get_ref())
    .await;

    match result {
        Ok(Some(user)) => {
            let parsed_hash = PasswordHash::new(&user.password);
            match parsed_hash {
                Ok(hash) => {
                    let valid = Argon2::default()
                        .verify_password(form.password.as_bytes(), &hash)
                        .is_ok();

                    if valid {
                        session.insert("user_id", user.id).unwrap();
                        session.insert("email", user.email.clone()).unwrap();
                        return HttpResponse::Ok().body("✅ Login correcto");
                    } else {
                        return HttpResponse::Unauthorized().body("❌ Contraseña incorrecta");
                    }
                }
                Err(_) => HttpResponse::InternalServerError().body("❌ Hash inválido"),
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("❌ Usuario no encontrado"),
        Err(e) => {
            eprintln!("Error BD: {:?}", e);
            HttpResponse::InternalServerError().body("❌ Error interno")
        }
    }
}

#[get("/me")]
pub async fn me(session: Session) -> HttpResponse {
    if let Some(email) = session.get::<String>("email").unwrap_or(None) {
        HttpResponse::Ok().json({
            serde_json::json!({
                "logged_in": true,
                "email": email
            })
        })
    } else {
        HttpResponse::Unauthorized().json({
            serde_json::json!({ "logged_in": false })
        })
    }
}