use actix_web::{web, App, HttpServer};
use auth_rs::{configuration::AppState, controllers::config, middlewares::RateLimitMiddleware};
use dotenv::dotenv;
use env_logger::Env;
use std::net::TcpListener;

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

    let rate_limit_config = app_state.get_app_config().get_rate_limit().clone();

    HttpServer::new(move || {
        let (rate_limiter, limiter) = RateLimitMiddleware::new(&rate_limit_config).unwrap();

        App::new()
            .wrap(rate_limiter)
            .app_data(limiter)
            .app_data(web::Data::new(app_state.clone()))
            .configure(config)
    })
    .workers(workers)
    .listen(listener)?
    .run()
    .await
}
