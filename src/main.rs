use actix_web::{web, App, HttpServer};
use auth_rs::{
    configuration::{AppState, RateLimitConfig},
    controllers::config,
    middlewares::RateLimitMiddleware,
};
use dotenv::dotenv;
use env_logger::Env;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_env();

    let app_state = AppState::new();
    let listener = create_listener(&app_state)?;
    let workers = app_state.get_app_config().get_server().get_workers();
    let rate_limit_config = app_state.get_app_config().get_rate_limit().clone();

    run_server(listener, workers, app_state, rate_limit_config).await
}

fn init_env() {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
}

fn create_listener(app_state: &AppState) -> std::io::Result<TcpListener> {
    let address = format!(
        "{}:{}",
        app_state.get_app_config().get_server().get_host(),
        app_state.get_app_config().get_server().get_port()
    );
    TcpListener::bind(address)
}

async fn run_server(
    listener: TcpListener,
    workers: usize,
    app_state: AppState,
    rate_limit_config: RateLimitConfig,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        let (rate_limiter, limiter) =
            RateLimitMiddleware::new(&rate_limit_config).expect("Failed to create rate limiter");

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
