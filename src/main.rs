use actix_web::{middleware::Logger, web, App, HttpServer};
use authnorization::{
    configuration::{ConfigBuilder, EnvReader},
    controller::Controllers,
};
use env_logger::Env;
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::{net::TcpListener, sync::Arc};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let env_reader = EnvReader::new();

    let config = env_reader
        .get_config()
        .expect("Failed to read configuration");

    let pool = PgPoolOptions::new().connect_lazy_with(config.db_config.to_pg_connect_options());
    let pool = Arc::new(pool);

    let config = ConfigBuilder::new()
        .server_config(config.server_config)
        .jwt_config(config.jwt_config)
        .email_config(config.email_config)
        .pool(pool.clone())
        .build()
        .expect("Failed to build configuration");

    let address = format!(
        "{}:{}",
        config.server_config.address(),
        config.server_config.port()
    );

    let listener = TcpListener::bind(&address)?;
    let worker_count = config.server_config.workers();

    info!("Listening on {}", &address);

    let config = web::Data::new(config);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(config.clone())
            .configure(Controllers::configure_routes)
    })
    .workers(worker_count)
    .listen(listener)?
    .run()
    .await
}
