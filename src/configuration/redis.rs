use std::str::FromStr;

use super::{read_env_var, ConfigBuilder, ConfigError, Result};

/// Represents the configuration for a Redis connection
#[derive(Debug, Clone)]
pub struct RedisConfig {
    username: Option<String>,
    password: Option<String>,
    port: u16,
    host: String,
    db_index: i64,
    tls_enabled: bool,
    connection_timeout: u64,
    keep_alive: bool,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            username: None,
            password: None,
            port: 6379,
            host: String::from("127.0.0.1"),
            db_index: 0,
            tls_enabled: false,
            connection_timeout: 5,
            keep_alive: true,
        }
    }
}

impl RedisConfig {
    pub fn get_username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    pub fn get_password(&self) -> Option<&str> {
        self.password.as_deref()
    }

    pub const fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub const fn get_db_index(&self) -> i64 {
        self.db_index
    }

    pub const fn is_tls_enabled(&self) -> bool {
        self.tls_enabled
    }

    pub const fn get_connection_timeout(&self) -> u64 {
        self.connection_timeout
    }

    pub const fn is_keep_alive_enabled(&self) -> bool {
        self.keep_alive
    }

    /// Converts the RedisConfig to a connection URL string
    pub fn to_connection_string(&self) -> String {
        let scheme = if self.tls_enabled { "rediss" } else { "redis" };
        let auth = match (&self.username, &self.password) {
            (Some(username), Some(password)) => format!("{}:{}@", username, password),
            (None, Some(password)) => format!("{}@", password),
            _ => String::new(),
        };

        format!(
            "{}://{}{}:{}/{}",
            scheme, auth, self.host, self.port, self.db_index
        )
    }

    /// Creates a ConnectionInfo struct for the redis crate
    pub fn to_connection_info(&self) -> Result<redis::ConnectionInfo> {
        let mut connection_info = redis::ConnectionInfo::from_str(&self.to_connection_string())?;

        if let Some(ref password) = self.password {
            connection_info.redis.password = Some(password.clone());
        }

        Ok(connection_info)
    }
}

/// A builder for `RedisConfig`
#[derive(Debug)]
pub struct RedisConfigBuilder {
    username: Option<String>,
    password: Option<String>,
    port: Option<u16>,
    host: Option<String>,
    db_index: Option<i64>,
    tls_enabled: Option<bool>,
    connection_timeout: Option<u64>,
    keep_alive: Option<bool>,
}

impl RedisConfigBuilder {
    pub const fn new() -> Self {
        Self {
            username: None,
            password: None,
            port: None,
            host: None,
            db_index: None,
            tls_enabled: None,
            connection_timeout: None,
            keep_alive: None,
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

    pub const fn with_db_index(mut self, db_index: i64) -> Self {
        self.db_index = Some(db_index);
        self
    }

    pub const fn with_tls_enabled(mut self, tls_enabled: bool) -> Self {
        self.tls_enabled = Some(tls_enabled);
        self
    }

    pub const fn with_connection_timeout(mut self, timeout: u64) -> Self {
        self.connection_timeout = Some(timeout);
        self
    }

    pub const fn with_keep_alive(mut self, keep_alive: bool) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }
}

impl ConfigBuilder for RedisConfigBuilder {
    type Config = RedisConfig;

    fn build(&self) -> Self::Config {
        let username = self
            .username
            .clone()
            .or_else(|| match read_env_var("REDIS_USERNAME") {
                Ok(u) => Some(u),
                Err(e) => {
                    log::debug!("{}. Using no username", e);
                    None
                }
            });

        let password = self
            .password
            .clone()
            .or_else(|| match read_env_var("REDIS_PASSWORD") {
                Ok(p) => Some(p),
                Err(e) => {
                    log::debug!("{}. Using no password", e);
                    None
                }
            });

        let port = self
            .port
            .unwrap_or_else(|| match read_env_var("REDIS_PORT") {
                Ok(p) => p.parse().unwrap_or_else(|e| {
                    log::warn!(
                        "{}. Using default {}",
                        ConfigError::from_parse_int_error("REDIS_PORT", e),
                        RedisConfig::default().port
                    );
                    RedisConfig::default().port
                }),
                Err(e) => {
                    log::warn!("{}. Using default {}", e, RedisConfig::default().port);
                    RedisConfig::default().port
                }
            });

        let host = self
            .host
            .clone()
            .unwrap_or_else(|| match read_env_var("REDIS_HOST") {
                Ok(h) => h,
                Err(e) => {
                    log::warn!("{}. Using default {}", e, RedisConfig::default().host);
                    RedisConfig::default().host
                }
            });

        let db_index = self
            .db_index
            .unwrap_or_else(|| match read_env_var("REDIS_DB_INDEX") {
                Ok(d) => d.parse().unwrap_or_else(|e| {
                    log::warn!(
                        "{}. Using default {}",
                        ConfigError::from_parse_int_error("REDIS_DB_INDEX", e),
                        RedisConfig::default().db_index
                    );
                    RedisConfig::default().db_index
                }),
                Err(e) => {
                    log::warn!("{}. Using default {}", e, RedisConfig::default().db_index);
                    RedisConfig::default().db_index
                }
            });

        let tls_enabled =
            self.tls_enabled
                .unwrap_or_else(|| match read_env_var("REDIS_TLS_ENABLED") {
                    Ok(t) => t.parse().unwrap_or_else(|e| {
                        log::warn!(
                            "{}. Using default {}",
                            ConfigError::from_parse_bool_error("REDIS_TLS_ENABLED", e),
                            RedisConfig::default().tls_enabled
                        );
                        RedisConfig::default().tls_enabled
                    }),
                    Err(e) => {
                        log::warn!(
                            "{}. Using default {}",
                            e,
                            RedisConfig::default().tls_enabled
                        );
                        RedisConfig::default().tls_enabled
                    }
                });

        let connection_timeout = self.connection_timeout.unwrap_or_else(|| {
            match read_env_var("REDIS_CONNECTION_TIMEOUT") {
                Ok(t) => t.parse().unwrap_or_else(|e| {
                    log::warn!(
                        "{}. Using default {}",
                        ConfigError::from_parse_int_error("REDIS_CONNECTION_TIMEOUT", e),
                        RedisConfig::default().connection_timeout
                    );
                    RedisConfig::default().connection_timeout
                }),
                Err(e) => {
                    log::warn!(
                        "{}. Using default {}",
                        e,
                        RedisConfig::default().connection_timeout
                    );
                    RedisConfig::default().connection_timeout
                }
            }
        });

        let keep_alive =
            self.keep_alive
                .unwrap_or_else(|| match read_env_var("REDIS_KEEP_ALIVE") {
                    Ok(k) => k.parse().unwrap_or_else(|e| {
                        log::warn!(
                            "{}. Using default {}",
                            ConfigError::from_parse_bool_error("REDIS_KEEP_ALIVE", e),
                            RedisConfig::default().keep_alive
                        );
                        RedisConfig::default().keep_alive
                    }),
                    Err(e) => {
                        log::warn!("{}. Using default {}", e, RedisConfig::default().keep_alive);
                        RedisConfig::default().keep_alive
                    }
                });

        RedisConfig {
            username,
            password,
            port,
            host,
            db_index,
            tls_enabled,
            connection_timeout,
            keep_alive,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = RedisConfig::default();
        assert_eq!(config.port, 6379);
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.db_index, 0);
        assert!(!config.tls_enabled);
        assert_eq!(config.connection_timeout, 5);
        assert!(config.keep_alive);
    }

    #[test]
    fn test_connection_string_no_auth() {
        let config = RedisConfig::default();
        assert_eq!(config.to_connection_string(), "redis://127.0.0.1:6379/0");
    }

    #[test]
    fn test_connection_string_with_password() {
        let config = RedisConfig {
            password: Some("secret".to_string()),
            ..RedisConfig::default()
        };
        assert_eq!(
            config.to_connection_string(),
            "redis://secret@127.0.0.1:6379/0"
        );
    }

    #[test]
    fn test_connection_string_with_tls() {
        let config = RedisConfig {
            tls_enabled: true,
            ..RedisConfig::default()
        };
        assert_eq!(config.to_connection_string(), "rediss://127.0.0.1:6379/0");
    }

    #[test]
    fn test_builder_pattern() {
        let config = RedisConfigBuilder::new()
            .with_host("redis.example.com")
            .with_port(6380)
            .with_password("secret")
            .with_db_index(1)
            .with_tls_enabled(true)
            .build();

        assert_eq!(config.host, "redis.example.com");
        assert_eq!(config.port, 6380);
        assert_eq!(config.password, Some("secret".to_string()));
        assert_eq!(config.db_index, 1);
        assert!(config.tls_enabled);
    }
}
