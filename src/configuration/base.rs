use std::{
    env::{var, VarError},
    num::ParseIntError,
};

use thiserror::Error;

use super::{
    DatabaseConfig, DatabaseConfigBuilder, Environment, JWTConfig, JWTConfigBuilder, ServerConfig,
    ServerConfigBuilder,
};

/// Represents possible configuration-related errors.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Configuration `{0}` is not set")]
    NotPresent(&'static str),

    #[error("Environment variable `{0}` contains invalid Unicode")]
    NotUnicode(&'static str),

    #[error("Failed to parse environment variable `{0}` into type {1}")]
    ParseError(&'static str, String),
}

impl ConfigError {
    /// Converts a `ParseIntError` into a `ConfigError` for improved error context.
    pub fn from_parse_int_error(var_name: &'static str, e: ParseIntError) -> Self {
        Self::ParseError(var_name, e.to_string())
    }
}

/// Alias for results returning configuration.
pub type Result<T> = std::result::Result<T, ConfigError>;

/// Defines the interface for types that can build configurations.
pub trait ConfigBuilder {
    type Config;
    /// Builds the configuration.
    fn build(&self) -> Result<Self::Config>;
}

/// Reads an environment variable, converting `env::var` errors into `ConfigError`.
pub fn read_env_var(var_name: &'static str) -> Result<String> {
    var(var_name).map_err(|e| match e {
        VarError::NotPresent => ConfigError::NotPresent(var_name),
        VarError::NotUnicode(_) => ConfigError::NotUnicode(var_name),
    })
}

/// Holds the complete application configuration.
#[derive(Debug, Clone)]
pub struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
    environment: Environment,
    jwt: JWTConfig,
}

impl AppConfig {
    /// Accessors for configuration components
    pub fn get_server(&self) -> &ServerConfig {
        &self.server
    }
    pub fn get_database(&self) -> &DatabaseConfig {
        &self.database
    }
    pub fn get_environment(&self) -> Environment {
        self.environment
    }
    pub fn get_jwt(&self) -> &JWTConfig {
        &self.jwt
    }
}

/// Builder for `AppConfig`, combining multiple configuration types.
#[derive(Debug, Default)]
pub struct AppConfigBuilder {
    server_builder: Option<ServerConfigBuilder>,
    database_builder: Option<DatabaseConfigBuilder>,
    environment: Option<Environment>,
    jwt_builder: Option<JWTConfigBuilder>,
}

impl AppConfigBuilder {
    /// Creates a new `AppConfigBuilder` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_server(mut self, builder: ServerConfigBuilder) -> Self {
        self.server_builder = Some(builder);
        self
    }

    pub fn with_database(mut self, builder: DatabaseConfigBuilder) -> Self {
        self.database_builder = Some(builder);
        self
    }

    pub fn with_environment(mut self, environment: Environment) -> Self {
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
    fn build(&self) -> Result<Self::Config> {
        let server = self
            .server_builder
            .as_ref()
            .ok_or(ConfigError::NotPresent("ServerConfig"))?
            .build()?;

        let database = self
            .database_builder
            .as_ref()
            .ok_or(ConfigError::NotPresent("DatabaseConfig"))?
            .build()?;

        let environment = self.environment.unwrap_or_else(|| {
            read_env_var("ENVIRONMENT")
                .ok()
                .and_then(|env| Environment::try_from(env.as_str()).ok())
                .unwrap_or_default()
        });

        let jwt = self
            .jwt_builder
            .as_ref()
            .ok_or(ConfigError::NotPresent("JWTConfig"))?
            .build()?;

        Ok(AppConfig {
            server,
            database,
            environment,
            jwt,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::SSLMode;

    // Helper function for setting up default AppConfig with all fields
    fn build_default_app_config() -> AppConfig {
        AppConfigBuilder::new()
            .with_server(
                ServerConfigBuilder::new()
                    .with_host("localhost")
                    .with_port(8080)
                    .with_workers(4),
            )
            .with_database(
                DatabaseConfigBuilder::new()
                    .with_host("localhost")
                    .with_port(5432)
                    .with_name("test")
                    .with_username("testuser")
                    .with_password("testpass")
                    .with_ssl_mode(SSLMode::Disable),
            )
            .with_jwt(
                JWTConfigBuilder::new()
                    .with_secret("secret")
                    .with_expiration_days(30),
            )
            .build()
            .unwrap()
    }

    #[test]
    fn app_config_builds_correctly() {
        let app_config = build_default_app_config();

        assert_eq!(app_config.get_server().get_host(), "localhost");
        assert_eq!(app_config.get_server().get_port(), 8080);
        assert_eq!(app_config.get_server().get_workers(), 4);

        assert_eq!(app_config.get_database().get_host(), "localhost");
        assert_eq!(app_config.get_database().get_port(), 5432);
        assert_eq!(app_config.get_database().get_name(), "test");
        assert_eq!(app_config.get_database().get_username(), "testuser");
        assert_eq!(app_config.get_database().get_password(), "testpass");
        assert_eq!(app_config.get_database().get_ssl_mode(), &SSLMode::Disable);

        assert_eq!(app_config.get_environment(), Environment::Development);
        assert_eq!(app_config.get_jwt().get_secret(), "secret");
        assert_eq!(app_config.get_jwt().get_expiration_days(), 30);
    }

    #[test]
    fn app_config_respects_env_variable() {
        std::env::set_var("ENVIRONMENT", "production");

        let app_config = build_default_app_config();

        assert_eq!(app_config.get_environment(), Environment::Production);
        std::env::remove_var("ENVIRONMENT");
    }

    #[test]
    fn missing_config_throws_error() {
        let result = AppConfigBuilder::new()
            .with_server(
                ServerConfigBuilder::new()
                    .with_host("localhost")
                    .with_port(8080)
                    .with_workers(4),
            )
            .build();

        assert!(matches!(
            result,
            Err(ConfigError::NotPresent("DatabaseConfig"))
        ));
    }
}
