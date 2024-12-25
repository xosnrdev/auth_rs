use auth::{
    bootstrap::run_application,
    utils::{AppResult, CONFIG},
};

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv::dotenv().ok();
    run_application(CONFIG.to_owned()).await
}
