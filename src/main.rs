use actix_web::{middleware::Logger, web, App, HttpServer};
use authnorization::{configuration::EnvReader, controller::Controllers};
use env_logger::Env;
use log::info;
use std::{net::TcpListener, sync::Arc};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = Arc::new(EnvReader::read_configuration().await);

    let address = format!(
        "{}:{}",
        config.server_config.address(),
        config.server_config.port()
    );

    let listener = TcpListener::bind(&address)?;
    let worker_count = config.server_config.workers();

    info!("Listening on {}", &address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(Arc::clone(&config)))
            .configure(Controllers::configure_routes)
    })
    .workers(worker_count)
    .listen(listener)?
    .run()
    .await
}
