use log::info;
use std::env;

use crate::configuration::{Config, DbConfig, JwtConfig, ServerConfig, SslMode};

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
    pub async fn read_configuration() -> Result<Config, Error> {
        info!("Reading configuration from environment variables");

        let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0".to_string());

        let server_port = get_env_var("SERVER_PORT", Some(8000))?;
        let server_workers = get_env_var("SERVER_WORKERS", Some(0))?;

        let db_username =
            env::var("DB_USERNAME").map_err(|_| Error::MissingEnvVar("DB_USERNAME".to_string()))?;
        let db_password =
            env::var("DB_PASSWORD").map_err(|_| Error::MissingEnvVar("DB_PASSWORD".to_string()))?;
        let db_port = get_env_var("DB_PORT", Some(5432))?;
        let db_host =
            env::var("DB_HOST").map_err(|_| Error::MissingEnvVar("DB_HOST".to_string()))?;
        let db_name =
            env::var("DB_NAME").map_err(|_| Error::MissingEnvVar("DB_NAME".to_string()))?;

        let db_ssl_mode = match env::var("DB_SSL_MODE") {
            Ok(d) => match d.as_str() {
                "require" => SslMode::Require,
                "prefer" => SslMode::Prefer,
                "disable" => SslMode::Disable,
                _ => return Err(Error::InvalidSslMode(d)),
            },
            Err(_) => SslMode::Require,
        };

        let jwt_secret =
            env::var("JWT_SECRET").map_err(|_| Error::MissingEnvVar("JWT_SECRET".to_string()))?;
        let jwt_expiration = get_env_var("JWT_EXPIRATION", Some(3600))?;

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

        Ok(Config::new(server_config, db_config, jwt_config).await)
    }
}

fn get_env_var<T: std::str::FromStr>(var_name: &str, default: Option<T>) -> Result<T, Error> {
    env::var(var_name)
        .map_err(|_| Error::MissingEnvVar(var_name.to_string()))
        .and_then(|val| {
            val.trim()
                .parse()
                .map_err(|_| Error::InvalidEnvVar(var_name.to_string()))
        })
        .or_else(|_| default.ok_or(Error::MissingEnvVar(var_name.to_string())))
}
