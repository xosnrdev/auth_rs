use actix_web::{web, App, HttpServer};
use authnorization::handler::config;
use dotenv::dotenv;
use env_logger::Env;
use sqlx::PgPool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
