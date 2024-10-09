use crate::configuration::{DbConfig, JwtConfig, ServerConfig, SslMode};
use log::{error, info};
use std::env;

use super::{ConfigOptions, EmailConfig};

#[derive(Debug, thiserror::Error)]
pub enum EnvReaderError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid environment variable: {0}")]
    InvalidEnvVar(String),
    #[error("Invalid SSL mode: {0}")]
    InvalidSslMode(String),
}

pub struct EnvReader;

impl EnvReader {
    pub fn new() -> Self {
        Self
    }

    pub fn get_config(&self) -> Result<ConfigOptions, EnvReaderError> {
        let server_config = self.get_server_config()?;
        let db_config = self.get_db_config()?;
        let jwt_config = self.get_jwt_config()?;
        let email_config = self.get_email_config()?;

        Ok(ConfigOptions {
            server_config,
            db_config,
            jwt_config,
            email_config,
        })
    }

    fn get_server_config(&self) -> Result<ServerConfig, EnvReaderError> {
        let server_addr = self.get_env_var("SERVER_ADDR", Some("0.0.0.0".to_string()))?;
        let server_port = self.get_env_var("SERVER_PORT", Some(8000))?;
        let server_workers = self.get_env_var("SERVER_WORKERS", Some(4))?;

        Ok(ServerConfig::new(server_addr, server_port, server_workers))
    }

    fn get_db_config(&self) -> Result<DbConfig, EnvReaderError> {
        let username = self.get_env_var("DB_USERNAME", None)?;
        let password = self.get_env_var("DB_PASSWORD", None)?;
        let host = self.get_env_var("DB_HOST", None)?;
        let port = self.get_env_var("DB_PORT", Some(5432))?;
        let name = self.get_env_var("DB_NAME", None)?;
        let get_ssl_mode = self.get_env_var("DB_SSL_MODE", Some("require".to_string()))?;

        let ssl_mode = match get_ssl_mode.to_lowercase().as_str() {
            "require" => SslMode::Require,
            "prefer" => SslMode::Prefer,
            "disable" => SslMode::Disable,
            _ => {
                return Err(EnvReaderError::InvalidSslMode(get_ssl_mode));
            }
        };

        Ok(DbConfig::new(
            username, password, port, host, name, ssl_mode,
        ))
    }

    fn get_jwt_config(&self) -> Result<JwtConfig, EnvReaderError> {
        let jwt_secret = self.get_env_var("JWT_SECRET", None)?;
        let jwt_expiration = self.get_env_var("JWT_EXPIRATION", Some(3600))?;

        Ok(JwtConfig::new(jwt_secret, jwt_expiration))
    }

    fn get_email_config(&self) -> Result<EmailConfig, EnvReaderError> {
        let server_token = self.get_env_var("SERVER_TOKEN", None)?;
        let smtp_host = self.get_env_var("SMTP_HOST", None)?;
        let sender = self.get_env_var("SENDER", None)?;
        let verification_token_expiration =
            self.get_env_var("VERIFICATION_TOKEN_EXPIRATION", Some(3600))?;

        Ok(EmailConfig::new(
            server_token,
            smtp_host,
            sender,
            verification_token_expiration,
        ))
    }

    fn parse_env_var<T>(&self, var_name: &str, value: String) -> Result<T, EnvReaderError>
    where
        T: std::str::FromStr + std::fmt::Debug,
    {
        value
            .parse()
            .map_err(|_| EnvReaderError::InvalidEnvVar(var_name.to_string()))
    }

    fn get_env_var<T>(&self, var_name: &str, default: Option<T>) -> Result<T, EnvReaderError>
    where
        T: std::str::FromStr + std::fmt::Debug,
    {
        match env::var(var_name) {
            Ok(val) => self.parse_env_var(var_name, val),
            Err(_) => {
                if let Some(d) = default {
                    info!("Using default value for {}: {:?}", var_name, d);
                    Ok(d)
                } else {
                    error!("Missing required environment variable: {}", var_name);
                    Err(EnvReaderError::MissingEnvVar(var_name.to_string()))
                }
            }
        }
    }
}
