use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;

mod handlers; // Asegúrate de tener mod handlers con submódulo auth

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL no está seteado");
    let db_pool = PgPool::connect(&db_url).await.expect("No se pudo conectar a DB");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(actix_cors::Cors::permissive()) // 🌐 Permitir CORS completo
            .service(handlers::auth::register_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
