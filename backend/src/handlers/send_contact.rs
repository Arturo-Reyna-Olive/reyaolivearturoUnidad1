use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
pub struct ContactForm {
    pub nombre: String,
    pub correo: String,
    pub mensaje: String,
}

#[post("/send-contact")]
pub async fn send_contact(form: web::Json<ContactForm>) -> impl Responder {
    let contenido = format!(
        "Mensaje de: {} ({})\n\nMensaje:\n{}",
        form.nombre, form.correo, form.mensaje
    );

    let result = Command::new("python")
        .arg("send_mail.py")
        .arg("arturoreyna694@gmail.com")
        .arg(&contenido)
        .output();

    match result {
        Ok(output) if output.status.success() => {
            HttpResponse::Ok().body("✅ Mensaje enviado correctamente.")
        }
        Ok(output) => {
            let err = String::from_utf8_lossy(&output.stderr);
            HttpResponse::InternalServerError().body(format!("❌ Error al enviar correo: {}", err))
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("❌ Error al ejecutar el script: {}", e))
        }
    }
}
