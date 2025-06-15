use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
pub struct RecoverInput {
    pub email: String,
}

#[post("/recover")]
pub async fn recover_password(
    db: web::Data<PgPool>,
    form: web::Json<RecoverInput>,
) -> HttpResponse {
    let email = form.email.trim().to_lowercase();

    // Verifica si el correo existe
    let result = sqlx::query("SELECT id FROM users WHERE email = $1")
        .bind(&email)
        .fetch_optional(db.get_ref())
        .await;

    match result {
        Ok(Some(_)) => {
            
            let token = "abc123";

            // ✅ IMPORTANTE: NO uses "index.html", usa solo "/#reset"
            let reset_url = format!("http://localhost:5500/#reset?token={}", token);

            // Llama al script de Python
            let status = Command::new("python")
                .arg("C:\\Genomatrix\\reyaolivearturoUnidad1\\mail\\send_mail.py")
                .arg(&email)
                .arg(&reset_url)
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("✅ Correo enviado correctamente");
                    HttpResponse::Ok().body("📬 Enlace de recuperación enviado")
                }
                Ok(s) => {
                    eprintln!("⚠️ Python terminó con código: {:?}", s.code());
                    HttpResponse::InternalServerError().body("❌ Error enviando el correo")
                }
                Err(e) => {
                    eprintln!("❌ Error ejecutando Python: {:?}", e);
                    HttpResponse::InternalServerError().body("❌ No se pudo ejecutar el script")
                }
            }
        }
        Ok(None) => HttpResponse::BadRequest().body("❌ Correo no registrado"),
        Err(e) => {
            eprintln!("❌ Error consultando usuario: {:?}", e);
            HttpResponse::InternalServerError().body("❌ Error interno")
        }
    }
}
