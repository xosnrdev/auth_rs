use super::{read_env_var, ConfigBuilder, ConfigError, Result};
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

impl DatabaseConfig {
    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_ssl_mode(&self) -> &SSLMode {
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
#[derive(Debug, Clone, PartialEq)]
pub enum SSLMode {
    Require,
    Prefer,
    Disable,
}

impl Default for SSLMode {
    fn default() -> Self {
        SSLMode::Prefer
    }
}

impl TryFrom<&str> for SSLMode {
    type Error = ConfigError;

    fn try_from(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "require" => Ok(Self::Require),
            "prefer" => Ok(Self::Prefer),
            "disable" => Ok(Self::Disable),
            _ => Err(ConfigError::ParseError("SSL_MODE", s.into())),
        }
    }
}

impl SSLMode {
    /// Converts SSLMode to PgSslMode
    fn to_pg_ssl_mode(&self) -> PgSslMode {
        match self {
            SSLMode::Require => PgSslMode::Require,
            SSLMode::Prefer => PgSslMode::Prefer,
            SSLMode::Disable => PgSslMode::Disable,
        }
    }
}

/// A builder for `DatabaseConfig`
#[derive(Debug, Default)]
pub struct DatabaseConfigBuilder {
    username: Option<String>,
    password: Option<String>,
    port: Option<u16>,
    host: Option<String>,
    name: Option<String>,
    ssl_mode: Option<SSLMode>,
}

impl DatabaseConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
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

    pub fn with_ssl_mode(mut self, ssl_mode: SSLMode) -> Self {
        self.ssl_mode = Some(ssl_mode);
        self
    }
}

impl ConfigBuilder for DatabaseConfigBuilder {
    type Config = DatabaseConfig;

    fn build(&self) -> Result<Self::Config> {
        let username = self
            .username
            .clone()
            .unwrap_or_else(|| read_env_var("DATABASE_USERNAME").unwrap_or_default());
        let password = self
            .password
            .clone()
            .unwrap_or_else(|| read_env_var("DATABASE_PASSWORD").unwrap_or_default());
        let port = self.port.unwrap_or_else(|| {
            read_env_var("DATABASE_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5432)
        });
        let host = self
            .host
            .clone()
            .unwrap_or_else(|| read_env_var("DATABASE_HOST").unwrap_or_default());
        let name = self
            .name
            .clone()
            .unwrap_or_else(|| read_env_var("DATABASE_NAME").unwrap_or_default());
        let ssl_mode = self.ssl_mode.clone().unwrap_or_else(|| {
            read_env_var("DATABASE_SSL_MODE")
                .ok()
                .and_then(|s| SSLMode::try_from(s.as_str()).ok())
                .unwrap_or_default()
        });

        Ok(DatabaseConfig {
            username,
            password,
            port,
            host,
            name,
            ssl_mode,
        })
    }
}
