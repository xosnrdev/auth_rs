use super::{read_env_var, ConfigBuilder, Result};
use crate::configuration::ConfigError;

#[derive(Debug, Clone, Default)]
pub struct JWTConfig {
    secret: String,
    expiration_days: i64,
}

impl JWTConfig {
    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    pub fn get_expiration_days(&self) -> i64 {
        self.expiration_days
    }
}

/// A builder for `JWTConfig`
#[derive(Debug, Default)]
pub struct JWTConfigBuilder {
    config: JWTConfig,
}

impl JWTConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_secret(mut self, secret: impl Into<String>) -> Self {
        self.config.secret = secret.into();
        self
    }

    pub fn with_expiration_days(mut self, expiration_days: i64) -> Self {
        self.config.expiration_days = expiration_days;
        self
    }
}

impl ConfigBuilder for JWTConfigBuilder {
    type Config = JWTConfig;

    fn build(&self) -> Result<Self::Config> {
        let secret = if !self.config.secret.is_empty() {
            self.config.secret.clone()
        } else {
            read_env_var("JWT_SECRET")?
        };

        let expiration_days = if self.config.expiration_days != 0 {
            self.config.expiration_days
        } else {
            read_env_var("JWT_EXPIRATION_DAYS")?
                .parse()
                .map_err(|e| ConfigError::from_parse_int_error("JWT_EXPIRATION_DAYS", e))?
        };

        Ok(JWTConfig {
            secret,
            expiration_days,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_config_builder() {
        let config = JWTConfigBuilder::new()
            .with_secret("test_secret")
            .with_expiration_days(30)
            .build()
            .unwrap();

        assert_eq!(config.get_secret(), "test_secret");
        assert_eq!(config.get_expiration_days(), 30);
    }

    #[test]
    fn test_jwt_config_builder_with_env() {
        std::env::set_var("JWT_SECRET", "env_secret");
        std::env::set_var("JWT_EXPIRATION_DAYS", "60");

        let config = JWTConfigBuilder::new().build().unwrap();

        assert_eq!(config.get_secret(), "env_secret");
        assert_eq!(config.get_expiration_days(), 60);

        // Clean up environment variables
        std::env::remove_var("JWT_SECRET");
        std::env::remove_var("JWT_EXPIRATION_DAYS");
    }

    #[test]
    fn test_jwt_config_builder_mixed() {
        std::env::set_var("JWT_SECRET", "env_secret");

        let config = JWTConfigBuilder::new()
            .with_expiration_days(45)
            .build()
            .unwrap();

        assert_eq!(config.get_secret(), "env_secret");
        assert_eq!(config.get_expiration_days(), 45);

        // Clean up environment variables
        std::env::remove_var("JWT_SECRET");
    }
}
