use sqlx::PgPool;
use std::env;

pub async fn init() -> Result<PgPool, sqlx::Error> {
    let url = env::var("DATABASE_URL").expect("Falta DATABASE_URL");
    PgPool::connect(&url).await
}
