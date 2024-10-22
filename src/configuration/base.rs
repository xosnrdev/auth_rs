use std::{
    env::{var, VarError},
    num::ParseIntError,
    process,
};

use thiserror::Error;

use super::{
    DatabaseConfig, DatabaseConfigBuilder, Environment, JWTConfig, JWTConfigBuilder, ServerConfig,
    ServerConfigBuilder,
};

/// Represents possible configuration-related errors.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Environment variable `{0}` not found")]
    NotPresent(&'static str),

    #[error("Environment variable `{0}` contains invalid Unicode")]
    NotUnicode(&'static str),

    #[error("Failed to parse environment variable `{0}:{1}`")]
    ParseError(&'static str, String),
}

impl ConfigError {
    pub fn from_parse_int_error(var_name: &'static str, e: ParseIntError) -> Self {
        ConfigError::ParseError(var_name, e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ConfigError>;

/// Defines the interface for types that can build configurations.
pub trait ConfigBuilder {
    type Config;
    /// Builds the configuration.
    fn build(&self) -> Self::Config;
}

/// Reads an environment variable, converting `env::var` errors into `ConfigError`.
pub fn read_env_var(var_name: &'static str) -> Result<String> {
    var(var_name).map_err(|e| match e {
        VarError::NotPresent => ConfigError::NotPresent(var_name),
        VarError::NotUnicode(_) => ConfigError::NotUnicode(var_name),
    })
}

/// Holds the complete application configuration.
#[derive(Clone)]
pub struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
    environment: Environment,
    jwt: JWTConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            environment: Environment::Development,
            jwt: JWTConfig::default(),
        }
    }
}

impl AppConfig {
    /// Accessors for configuration components
    pub const fn get_server(&self) -> &ServerConfig {
        &self.server
    }
    pub const fn get_database(&self) -> &DatabaseConfig {
        &self.database
    }
    pub const fn get_environment(&self) -> Environment {
        self.environment
    }
    pub const fn get_jwt(&self) -> &JWTConfig {
        &self.jwt
    }
}

/// Builder for `AppConfig`, combining multiple configuration types.
pub struct AppConfigBuilder {
    server_builder: Option<ServerConfigBuilder>,
    database_builder: Option<DatabaseConfigBuilder>,
    environment: Option<Environment>,
    jwt_builder: Option<JWTConfigBuilder>,
}

impl AppConfigBuilder {
    pub const fn new() -> Self {
        Self {
            server_builder: None,
            database_builder: None,
            environment: None,
            jwt_builder: None,
        }
    }

    pub fn with_server(mut self, builder: ServerConfigBuilder) -> Self {
        self.server_builder = Some(builder);
        self
    }

    pub fn with_database(mut self, builder: DatabaseConfigBuilder) -> Self {
        self.database_builder = Some(builder);
        self
    }

    pub const fn with_environment(mut self, environment: Environment) -> Self {
        self.environment = Some(environment);
        self
    }

    pub fn with_jwt(mut self, builder: JWTConfigBuilder) -> Self {
        self.jwt_builder = Some(builder);
        self
    }
}

impl ConfigBuilder for AppConfigBuilder {
    type Config = AppConfig;

    /// Builds the complete `AppConfig`, combining all sub-configurations.
    fn build(&self) -> Self::Config {
        let server = match *&self.server_builder {
            Some(ref builder) => builder.build(),
            None => {
                log::warn!("ServerConfig not set. Using default configuration");
                AppConfig::default().server
            }
        };

        let database = match *&self.database_builder {
            Some(ref builder) => builder.build(),
            None => {
                log::error!("DatabaseConfig not set");
                process::exit(1);
            }
        };

        let environment = self
            .environment
            .unwrap_or_else(|| match read_env_var("ENVIRONMENT") {
                Ok(env) => Environment::try_from(env).unwrap_or_else(|e| {
                    log::warn!(
                        "{}. Using default {}",
                        e,
                        AppConfig::default().environment.as_str()
                    );
                    AppConfig::default().environment
                }),
                Err(e) => {
                    log::warn!(
                        "{}. Using default {}",
                        e,
                        AppConfig::default().environment.as_str()
                    );
                    AppConfig::default().environment
                }
            });

        let jwt = match *&self.jwt_builder {
            Some(ref builder) => builder.build(),
            None => {
                log::error!("JWTConfig not set");
                process::exit(1);
            }
        };

        AppConfig {
            server,
            database,
            environment,
            jwt,
        }
    }
}
