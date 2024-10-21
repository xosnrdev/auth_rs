use super::{read_env_var, ConfigBuilder, Result};

#[derive(Debug, Clone, Default)]
pub struct JWTConfig {
    secret: String,
    expiration_days: i64,
}

impl JWTConfig {
    pub fn new(secret: String, expiration_days: i64) -> Self {
        Self {
            secret,
            expiration_days,
        }
    }
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
    secret: Option<String>,
    expiration_days: Option<i64>,
}

impl JWTConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_secret(mut self, secret: impl Into<String>) -> Self {
        self.secret = Some(secret.into());
        self
    }

    pub fn with_expiration_days(mut self, expiration_days: i64) -> Self {
        self.expiration_days = Some(expiration_days.into());
        self
    }
}

impl ConfigBuilder for JWTConfigBuilder {
    type Config = JWTConfig;

    fn build(&self) -> Result<Self::Config> {
        let secret = self
            .secret
            .clone()
            .unwrap_or_else(|| read_env_var("JWT_SECRET").unwrap_or_default());

        let expiration_days = self.expiration_days.unwrap_or_else(|| {
            read_env_var("JWT_EXPIRATION_DAYS")
                .ok()
                .and_then(|j| j.parse().ok())
                .unwrap_or(7)
        });

        Ok(JWTConfig {
            secret,
            expiration_days,
        })
    }
}
