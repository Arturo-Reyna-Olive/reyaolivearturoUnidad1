use actix_web::{web, App, HttpServer};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use dotenvy::dotenv;

mod db;
mod handlers;
mod models;
mod services;
mod utils;

use handlers::{auth, login, recover, send_contact};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let db_pool = db::init_db().await.expect("Error al conectar con la base de datos");
    let secret_key = Key::generate();

    println!("🚀 Backend corriendo en http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .wrap(actix_cors::Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .supports_credentials()
            )
            .service(
                web::scope("/auth")
                    .service(auth::register_user)
                    .service(login::login_user)
                    .service(login::me)
                    .service(recover::recover_password)
            )
            .service(
                web::scope("/")
                    .service(send_contact::send_contact) // aquí montamos la nueva ruta
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
