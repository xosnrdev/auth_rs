use std::net::TcpListener;

use actix_web::{web, App, HttpServer};
use authnorization::{configuration::AppState, controllers::config};
use dotenv::dotenv;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app_state = AppState::new();

    let address = format!(
        "{}:{}",
        app_state.get_app_config().get_server().get_host(),
        app_state.get_app_config().get_server().get_port()
    );
    let workers = app_state.get_app_config().get_server().get_workers();
    let listener = TcpListener::bind(address)?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(config)
    })
    .workers(workers)
    .listen(listener)?
    .run()
    .await
}
