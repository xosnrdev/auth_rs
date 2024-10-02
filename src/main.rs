use authnorization::configuration::EnvReader;
use env_logger::Env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let _config = EnvReader::read_configuration().await;

    Ok(println!("Hello, World!"))
}
