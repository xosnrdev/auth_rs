use crate::configuration::{Config, DbConfig, JwtConfig, ServerConfig, SslMode};
use log::{error, info};
use std::env;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid environment variable: {0}")]
    InvalidEnvVar(String),
    #[error("Invalid SSL mode: {0}")]
    InvalidSslMode(String),
}

pub struct EnvReader;

impl EnvReader {
    pub async fn read_configuration() -> Config {
        info!("Reading configuration from environment variables");

        let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| {
            info!("SERVER_ADDR not found, using default: 0.0.0.0");
            "0.0.0.0".to_string()
        });

        let server_port = get_required_env_var("SERVER_PORT").unwrap_or_else(|e| {
            error!("{}", e);
            panic!("{}", e);
        });

        let server_workers = get_env_var("SERVER_WORKERS", Some(0)).unwrap_or_else(|e| {
            error!("Invalid SERVER_WORKERS: {}. Using default 0", e);
            0
        });

        let db_username = get_required_env_var("DB_USERNAME").unwrap_or_else(|e| {
            error!("{}", e);
            panic!("{}", e);
        });

        let db_password = get_required_env_var("DB_PASSWORD").unwrap_or_else(|e| {
            error!("{}", e);
            panic!("{}", e);
        });

        let db_port = get_env_var("DB_PORT", Some(5432)).unwrap_or_else(|e| {
            error!("Invalid DB_PORT: {}. Using default 5432", e);
            5432
        });

        let db_host = get_required_env_var("DB_HOST").unwrap_or_else(|e| {
            error!("{}", e);
            panic!("{}", e);
        });

        let db_name = get_required_env_var("DB_NAME").unwrap_or_else(|e| {
            error!("{}", e);
            panic!("{}", e);
        });

        let db_ssl_mode = match env::var("DB_SSL_MODE") {
            Ok(d) => match d.as_str() {
                "require" => SslMode::Require,
                "prefer" => SslMode::Prefer,
                "disable" => SslMode::Disable,
                _ => {
                    error!("Invalid DB_SSL_MODE: {}. Using default: require", d);
                    SslMode::Require
                }
            },
            Err(_) => {
                info!("DB_SSL_MODE not found, using default: require");
                SslMode::Require
            }
        };

        let jwt_secret = get_required_env_var("JWT_SECRET").unwrap_or_else(|e| {
            error!("{}", e);
            panic!("{}", e);
        });

        let jwt_expiration = get_env_var("JWT_EXPIRATION", Some(3600)).unwrap_or_else(|e| {
            error!("Invalid JWT_EXPIRATION: {}. Using default 3600", e);
            3600
        });

        let db_config = DbConfig::new(
            db_username,
            db_password,
            db_port,
            db_host,
            db_name,
            db_ssl_mode,
        );
        let server_config = ServerConfig::new(server_addr, server_port, server_workers);
        let jwt_config = JwtConfig::new(jwt_secret, jwt_expiration);

        Config::new(server_config, db_config, jwt_config).await
    }
}

fn parse_env_var<T: std::str::FromStr>(var_name: &str, value: String) -> Result<T, Error> {
    value
        .trim()
        .parse()
        .map_err(|_| Error::InvalidEnvVar(var_name.to_string()))
}

fn get_env_var<T: std::str::FromStr>(var_name: &str, default: Option<T>) -> Result<T, Error> {
    env::var(var_name)
        .map(|val| parse_env_var(var_name, val))
        .unwrap_or_else(|_| default.ok_or(Error::MissingEnvVar(var_name.to_string())))
}

fn get_required_env_var<T: std::str::FromStr>(var_name: &str) -> Result<T, Error> {
    let val = env::var(var_name).map_err(|_| Error::MissingEnvVar(var_name.to_string()))?;
    parse_env_var(var_name, val)
}
