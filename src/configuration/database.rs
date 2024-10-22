use std::process;

use crate::configuration::ConfigError;

use super::{read_env_var, ConfigBuilder};
use sqlx::postgres::{PgConnectOptions, PgSslMode};

/// Represents the configuration for a database connection
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    username: String,
    password: String,
    port: u16,
    host: String,
    name: String,
    ssl_mode: SSLMode,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            port: 5432,
            host: String::from("127.0.0.1"),
            name: String::new(),
            ssl_mode: SSLMode::default(),
        }
    }
}

impl DatabaseConfig {
    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub const fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub const fn get_ssl_mode(&self) -> &SSLMode {
        &self.ssl_mode
    }

    /// Converts the DatabaseConfig to PgConnectOptions
    pub fn to_pg_connect_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .host(&self.host)
            .database(&self.name)
            .ssl_mode(self.ssl_mode.to_pg_ssl_mode())
    }
}

/// Represents the SSL mode for the database connection
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SSLMode {
    Require,
    #[default]
    Prefer,
    Disable,
}

impl TryFrom<String> for SSLMode {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "require" => Ok(Self::Require),
            "prefer" => Ok(Self::Prefer),
            "disable" => Ok(Self::Disable),
            other => Err(format!(
                "{} is not a supported ssl mode. Use either `require`, `prefer`, or `disable`",
                other
            )),
        }
    }
}

impl SSLMode {
    /// Converts SSLMode to PgSslMode
    const fn to_pg_ssl_mode(&self) -> PgSslMode {
        match *self {
            SSLMode::Require => PgSslMode::Require,
            SSLMode::Prefer => PgSslMode::Prefer,
            SSLMode::Disable => PgSslMode::Disable,
        }
    }

    const fn as_str(&self) -> &'static str {
        match *self {
            SSLMode::Require => "require",
            SSLMode::Prefer => "prefer",
            SSLMode::Disable => "disable",
        }
    }
}

/// A builder for `DatabaseConfig`
#[derive(Debug)]
pub struct DatabaseConfigBuilder {
    username: Option<String>,
    password: Option<String>,
    port: Option<u16>,
    host: Option<String>,
    name: Option<String>,
    ssl_mode: Option<SSLMode>,
}

impl DatabaseConfigBuilder {
    pub const fn new() -> Self {
        Self {
            username: None,
            password: None,
            port: None,
            host: None,
            name: None,
            ssl_mode: None,
        }
    }

    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub const fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub const fn with_ssl_mode(mut self, ssl_mode: SSLMode) -> Self {
        self.ssl_mode = Some(ssl_mode);
        self
    }
}

impl ConfigBuilder for DatabaseConfigBuilder {
    type Config = DatabaseConfig;

    fn build(&self) -> Self::Config {
        let username =
            self.username
                .clone()
                .unwrap_or_else(|| match read_env_var("DATABASE_USERNAME") {
                    Ok(u) => u,
                    Err(e) => {
                        log::error!("{}. Exiting...", e);
                        process::exit(1);
                    }
                });

        let password =
            self.password
                .clone()
                .unwrap_or_else(|| match read_env_var("DATABASE_PASSWORD") {
                    Ok(p) => p,
                    Err(e) => {
                        log::error!("{}. Exiting...", e);
                        process::exit(1);
                    }
                });

        let port = self
            .port
            .unwrap_or_else(|| match read_env_var("DATABASE_PORT") {
                Ok(p) => p.parse().unwrap_or_else(|e| {
                    log::warn!(
                        "{}. Using default {}",
                        ConfigError::from_parse_int_error("DATABASE_PORT", e),
                        DatabaseConfig::default().port
                    );
                    DatabaseConfig::default().port
                }),
                Err(e) => {
                    log::warn!("{}. Using default {}", e, DatabaseConfig::default().port);
                    DatabaseConfig::default().port
                }
            });

        let host = self
            .host
            .clone()
            .unwrap_or_else(|| match read_env_var("DATABASE_HOST") {
                Ok(h) => h,
                Err(e) => {
                    log::warn!("{}. Using default {}", e, DatabaseConfig::default().host);
                    DatabaseConfig::default().host
                }
            });

        let name = self
            .name
            .clone()
            .unwrap_or_else(|| match read_env_var("DATABASE_NAME") {
                Ok(n) => n,
                Err(e) => {
                    log::error!("{}. Exiting...", e);
                    process::exit(1);
                }
            });

        let ssl_mode =
            self.ssl_mode
                .clone()
                .unwrap_or_else(|| match read_env_var("DATABASE_SSL_MODE") {
                    Ok(s) => SSLMode::try_from(s).unwrap_or_else(|e| {
                        log::warn!(
                            "{}. Using default {}",
                            e,
                            DatabaseConfig::default().ssl_mode.as_str()
                        );
                        DatabaseConfig::default().ssl_mode
                    }),
                    Err(e) => {
                        log::warn!("{}", e);
                        DatabaseConfig::default().ssl_mode
                    }
                });

        DatabaseConfig {
            username,
            password,
            port,
            host,
            name,
            ssl_mode,
        }
    }
}
