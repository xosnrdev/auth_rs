use crate::configuration::ConfigError;

use super::{read_env_var, ConfigBuilder};

#[derive(Debug, Clone)]
pub struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
}

impl Default for ServerConfig {
    /// Returns a default `ServerConfig`
    fn default() -> Self {
        Self {
            host: String::from("0.0.0.0"),
            port: 50051,
            workers: 4,
        }
    }
}

impl ServerConfig {
    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub const fn get_port(&self) -> u16 {
        self.port
    }

    pub const fn get_workers(&self) -> usize {
        self.workers
    }
}

/// A builder for `ServerConfig`
pub struct ServerConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    workers: Option<usize>,
}

impl ServerConfigBuilder {
    pub const fn new() -> Self {
        Self {
            host: None,
            port: None,
            workers: None,
        }
    }

    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    pub const fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub const fn with_workers(mut self, workers: usize) -> Self {
        self.workers = Some(workers);
        self
    }
}

impl ConfigBuilder for ServerConfigBuilder {
    type Config = ServerConfig;

    fn build(&self) -> Self::Config {
        let host = self
            .host
            .clone()
            .unwrap_or_else(|| match read_env_var("SERVER_HOST") {
                Ok(host) => host,
                Err(e) => {
                    log::warn!("{}.  Using default {}", e, ServerConfig::default().host);
                    ServerConfig::default().host
                }
            });

        let port = self
            .port
            .unwrap_or_else(|| match read_env_var("SERVER_PORT") {
                Ok(port) => port.parse().unwrap_or_else(|e| {
                    log::warn!(
                        "{}. Using default {}",
                        ConfigError::from_parse_int_error("SERVER_PORT", e),
                        ServerConfig::default().port
                    );
                    ServerConfig::default().port
                }),
                Err(e) => {
                    log::warn!("{}. Using default {}", e, ServerConfig::default().port);
                    ServerConfig::default().port
                }
            });

        let workers = self
            .workers
            .unwrap_or_else(|| match read_env_var("SERVER_WORKERS") {
                Ok(workers) => workers.parse().unwrap_or_else(|e| {
                    log::warn!(
                        "{}. Using default {}",
                        ConfigError::from_parse_int_error("SERVER_WORKERS", e),
                        ServerConfig::default().workers
                    );
                    ServerConfig::default().workers
                }),
                Err(e) => {
                    log::warn!("{}. Using default {}", e, ServerConfig::default().workers);
                    ServerConfig::default().workers
                }
            });

        ServerConfig {
            host,
            port,
            workers,
        }
    }
}
