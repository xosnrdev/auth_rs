use super::{read_env_var, ConfigBuilder, Result};

#[derive(Debug, Clone, Default)]
pub struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
}

impl ServerConfig {
    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_workers(&self) -> usize {
        self.workers
    }
}

/// A builder for `ServerConfig`
#[derive(Debug, Default)]
pub struct ServerConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    workers: Option<usize>,
}

impl ServerConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn with_workers(mut self, workers: usize) -> Self {
        self.workers = Some(workers);
        self
    }
}

impl ConfigBuilder for ServerConfigBuilder {
    type Config = ServerConfig;

    fn build(&self) -> Result<Self::Config> {
        let host = self
            .host
            .clone()
            .unwrap_or_else(|| read_env_var("SERVER_HOST").unwrap_or("0.0.0.0".into()));

        let port = self.port.unwrap_or_else(|| {
            read_env_var("SERVER_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(50051)
        });

        let workers = self.workers.unwrap_or_else(|| {
            read_env_var("SERVER_WORKERS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10)
        });

        Ok(ServerConfig {
            host,
            port,
            workers,
        })
    }
}
