use std::process;

use super::{read_env_var, ConfigBuilder, ConfigError};

#[derive(Debug, Clone)]
pub struct JWTConfig {
    secret: String,
    access_token_duration_min: i64,
    refresh_token_duration_day: i64,
}

impl Default for JWTConfig {
    fn default() -> Self {
        Self {
            secret: String::new(),
            access_token_duration_min: 15,
            refresh_token_duration_day: 7,
        }
    }
}

impl JWTConfig {
    pub fn new(
        secret: String,
        refresh_token_duration_day: i64,
        access_token_duration_min: i64,
    ) -> Self {
        Self {
            secret,
            refresh_token_duration_day,
            access_token_duration_min,
        }
    }
    pub fn get_secret(&self) -> &str {
        &self.secret
    }

    pub const fn get_access_token_duration_min(&self) -> i64 {
        self.access_token_duration_min
    }

    pub const fn get_refresh_token_duration_day(&self) -> i64 {
        self.refresh_token_duration_day
    }
}

/// A builder for `JWTConfig`
#[derive(Debug)]
pub struct JWTConfigBuilder {
    secret: Option<String>,
    access_token_duration_min: Option<i64>,
    refresh_token_duration_day: Option<i64>,
}

impl JWTConfigBuilder {
    pub const fn new() -> Self {
        Self {
            secret: None,
            access_token_duration_min: None,
            refresh_token_duration_day: None,
        }
    }

    pub fn with_secret(mut self, secret: impl Into<String>) -> Self {
        self.secret = Some(secret.into());
        self
    }

    pub const fn with_access_token_duration_min(mut self, access_token_duration_min: i64) -> Self {
        self.access_token_duration_min = Some(access_token_duration_min);
        self
    }

    pub const fn with_refresh_token_duration_day(
        mut self,
        refresh_token_duration_day: i64,
    ) -> Self {
        self.refresh_token_duration_day = Some(refresh_token_duration_day);
        self
    }
}

impl ConfigBuilder for JWTConfigBuilder {
    type Config = JWTConfig;

    fn build(&self) -> Self::Config {
        let secret = self
            .secret
            .clone()
            .unwrap_or_else(|| match read_env_var("JWT_SECRET") {
                Ok(secret) => secret,
                Err(e) => {
                    log::error!("{}. Exiting...", e);
                    process::exit(1);
                }
            });

        let access_token_duration_min = self.access_token_duration_min.unwrap_or_else(|| {
            match read_env_var("JWT_ACCESS_TOKEN_DURATION_MIN") {
                Ok(v) => v.parse().unwrap_or_else(|e| {
                    log::warn!(
                        "{}. Using default {}",
                        ConfigError::from_parse_int_error("JWT_ACCESS_TOKEN_DURATION_MIN", e),
                        JWTConfig::default().get_access_token_duration_min()
                    );
                    JWTConfig::default().get_access_token_duration_min()
                }),
                Err(e) => {
                    log::warn!(
                        "{}. Using default {}",
                        e,
                        JWTConfig::default().get_access_token_duration_min()
                    );
                    JWTConfig::default().get_access_token_duration_min()
                }
            }
        });

        let refresh_token_duration_day = self.refresh_token_duration_day.unwrap_or_else(|| {
            match read_env_var("JWT_REFRESH_TOKEN_DURATION_DAY") {
                Ok(v) => v.parse().unwrap_or_else(|e| {
                    log::warn!(
                        "{}. Using default {}",
                        ConfigError::from_parse_int_error("JWT_REFRESH_TOKEN_DURATION_DAY", e),
                        JWTConfig::default().get_refresh_token_duration_day()
                    );
                    JWTConfig::default().get_refresh_token_duration_day()
                }),
                Err(e) => {
                    log::warn!(
                        "{}. Using default {}",
                        e,
                        JWTConfig::default().get_refresh_token_duration_day()
                    );
                    JWTConfig::default().get_refresh_token_duration_day()
                }
            }
        });

        JWTConfig {
            secret,
            access_token_duration_min,
            refresh_token_duration_day,
        }
    }
}
