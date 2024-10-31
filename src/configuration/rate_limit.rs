use super::{read_env_var, ConfigBuilder, ConfigError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// The key strategy to use for rate limiting.
/// - Token: Use the token as the key.
/// - IpAddress: Use the IP address as the key.
/// - TokenOrIpWithPath: Use the token as the key if it exists, otherwise use the IP address with the path.
pub enum KeyStrategy {
    #[default]
    Token,
    IpAddress,
    TokenOrIpWithPath,
}

impl KeyStrategy {
    pub fn as_str(&self) -> &'static str {
        match *self {
            self::KeyStrategy::Token => "token",
            self::KeyStrategy::IpAddress => "ip-address",
            self::KeyStrategy::TokenOrIpWithPath => "token-or-ip-with-path",
        }
    }
}

impl TryFrom<String> for KeyStrategy {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_ascii_lowercase().as_str(){
            "token" => Ok(Self::Token),
            "ip-address" => Ok(Self::IpAddress),
            "token-or-ip-with-path" => Ok(Self::TokenOrIpWithPath),
            other => Err(format!(
                "{} is not a supported key strategy. Use either `token`, `ip-address`, or `token-or-ip-with-path`",
                other
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    requests_per_window: usize,
    window_size: u64,
    error_message: Option<String>,
    redis_url: String,
    key_strategy: KeyStrategy,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_window: 100,
            window_size: 60,
            error_message: None,
            redis_url: String::from("redis://127.0.0.1"),
            key_strategy: KeyStrategy::default(),
        }
    }
}

impl RateLimitConfig {
    pub const fn get_requests_per_window(&self) -> usize {
        self.requests_per_window
    }

    pub const fn get_window_size(&self) -> u64 {
        self.window_size
    }

    pub fn get_error_message(&self) -> Option<&String> {
        self.error_message.as_ref()
    }

    pub fn get_redis_url(&self) -> &str {
        &self.redis_url
    }

    pub fn get_key_strategy(&self) -> &KeyStrategy {
        &self.key_strategy
    }
}

pub struct RateLimitConfigBuilder {
    requests_per_window: Option<usize>,
    window_size: Option<u64>,
    error_message: Option<String>,
    redis_url: Option<String>,
    key_strategy: Option<KeyStrategy>,
}

impl RateLimitConfigBuilder {
    pub const fn new() -> Self {
        Self {
            requests_per_window: None,
            window_size: None,
            error_message: None,
            redis_url: None,
            key_strategy: None,
        }
    }

    pub const fn with_requests_per_window(mut self, requests: usize) -> Self {
        self.requests_per_window = Some(requests);
        self
    }

    pub const fn with_window_size(mut self, seconds: u64) -> Self {
        self.window_size = Some(seconds);
        self
    }

    pub fn with_error_message(mut self, message: impl Into<String>) -> Self {
        self.error_message = Some(message.into());
        self
    }

    pub fn with_redis_url(mut self, url: impl Into<String>) -> Self {
        self.redis_url = Some(url.into());
        self
    }

    pub fn with_key_strategy(mut self, strategy: KeyStrategy) -> Self {
        self.key_strategy = Some(strategy);
        self
    }
}

impl ConfigBuilder for RateLimitConfigBuilder {
    type Config = RateLimitConfig;

    fn build(&self) -> Self::Config {
        let requests_per_window =
            self.requests_per_window
                .unwrap_or_else(|| match read_env_var("RATE_LIMIT_REQUESTS") {
                    Ok(requests) => requests.parse().unwrap_or_else(|e| {
                        log::warn!(
                            "{}. Using default {}",
                            ConfigError::from_parse_int_error("RATE_LIMIT_REQUESTS", e),
                            RateLimitConfig::default().requests_per_window
                        );
                        RateLimitConfig::default().requests_per_window
                    }),
                    Err(e) => {
                        log::warn!(
                            "{}. Using default {}",
                            e,
                            RateLimitConfig::default().requests_per_window
                        );
                        RateLimitConfig::default().requests_per_window
                    }
                });

        let window_size =
            self.window_size
                .unwrap_or_else(|| match read_env_var("RATE_LIMIT_WINDOW") {
                    Ok(window) => window.parse().unwrap_or_else(|e| {
                        log::warn!(
                            "{}. Using default {}",
                            ConfigError::from_parse_int_error("RATE_LIMIT_WINDOW", e),
                            RateLimitConfig::default().window_size
                        );
                        RateLimitConfig::default().window_size
                    }),
                    Err(e) => {
                        log::warn!(
                            "{}. Using default {}",
                            e,
                            RateLimitConfig::default().window_size
                        );
                        RateLimitConfig::default().window_size
                    }
                });

        let redis_url =
            self.redis_url
                .clone()
                .unwrap_or_else(|| match read_env_var("RATE_LIMIT_REDIS_URL") {
                    Ok(url) => url,
                    Err(e) => {
                        log::warn!(
                            "{}. Using default {}",
                            e,
                            RateLimitConfig::default().redis_url
                        );
                        RateLimitConfig::default().redis_url
                    }
                });

        let key_strategy = self.key_strategy.clone().unwrap_or_else(|| {
            match read_env_var("RATE_LIMIT_KEY_STRATEGY") {
                Ok(key) => KeyStrategy::try_from(key).unwrap_or_else(|e| {
                    log::warn!(
                        "{}. Using default {}",
                        e,
                        RateLimitConfig::default().key_strategy.as_str()
                    );
                    RateLimitConfig::default().key_strategy
                }),
                Err(e) => {
                    log::warn!(
                        "{}. Using default {}",
                        e,
                        RateLimitConfig::default().key_strategy.as_str()
                    );
                    RateLimitConfig::default().key_strategy
                }
            }
        });

        RateLimitConfig {
            requests_per_window,
            window_size,
            error_message: self.error_message.clone(),
            redis_url,
            key_strategy,
        }
    }
}
