use actix_limitation::{Error, Limiter, RateLimiter};

use actix_web::{dev::ServiceRequest, web};
use std::time::Duration;

use crate::{
    configuration::{KeyStrategy, RateLimitConfig},
    utils::TokenExtract,
};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct RateLimitError(#[from] Error);

#[derive(Debug)]
struct IpAddress(String);

impl IpAddress {
    fn new(req: &ServiceRequest) -> Self {
        Self(
            req.connection_info()
                .realip_remote_addr()
                .unwrap_or("127.0.0.1")
                .to_string(),
        )
    }
}

type Result<T> = std::result::Result<T, RateLimitError>;

struct KeyGenerator<'a> {
    request: &'a ServiceRequest,
    config: &'a RateLimitConfig,
    ip_address: IpAddress,
}

impl<'a> KeyGenerator<'a> {
    fn new(request: &'a ServiceRequest, config: &'a RateLimitConfig) -> Self {
        Self {
            request,
            config,
            ip_address: IpAddress::new(request),
        }
    }

    fn generate(&self) -> Result<String> {
        match *self.config.get_key_strategy() {
            KeyStrategy::Token => Ok(TokenExtract::extract(self.request.request())
                .ok()
                .unwrap_or(self.ip_address.0.clone())),
            KeyStrategy::IpAddress => Ok(self.ip_address.0.clone()),
            KeyStrategy::TokenOrIpWithPath => {
                let identifier = TokenExtract::extract(self.request.request())
                    .ok()
                    .unwrap_or(self.ip_address.0.clone());
                Ok(format!("{}:{}", identifier, self.request.path()))
            }
        }
    }
}

fn build_limiter(config: &RateLimitConfig) -> Result<Limiter> {
    let config_clone = config.clone();
    Ok(Limiter::builder(config.get_redis_url())
        .key_by(move |req: &ServiceRequest| KeyGenerator::new(req, &config_clone).generate().ok())
        .limit(config.get_requests_per_window())
        .period(Duration::from_secs(config.get_window_size()))
        .build()?)
}

pub struct RateLimitMiddleware;

impl RateLimitMiddleware {
    pub fn new(config: &RateLimitConfig) -> Result<(RateLimiter, web::Data<Limiter>)> {
        let limiter = build_limiter(config)?;

        Ok((RateLimiter::default(), web::Data::new(limiter)))
    }
}
