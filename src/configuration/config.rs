use super::{DbConfig, EmailConfig, JwtConfig, ServerConfig};
use crate::services::PasswordService;
use crate::{
    repositories::{EmailVerificationRepository, UserRepository},
    services::{EmailVerificationService, JwtService, Services, TokenService, UserService},
};
use sqlx::PgPool;
use std::sync::Arc;

pub struct ConfigOptions {
    pub server_config: ServerConfig,
    pub db_config: DbConfig,
    pub jwt_config: JwtConfig,
    pub email_config: EmailConfig,
}

pub struct Config {
    pub server_config: ServerConfig,
    pub services: Services,
}

pub struct ConfigBuilder {
    server_config: Option<ServerConfig>,
    jwt_config: Option<JwtConfig>,
    email_config: Option<EmailConfig>,
    pool: Option<Arc<PgPool>>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            server_config: None,
            jwt_config: None,
            email_config: None,
            pool: None,
        }
    }

    pub fn server_config(mut self, config: ServerConfig) -> Self {
        self.server_config = Some(config);
        self
    }

    pub fn jwt_config(mut self, config: JwtConfig) -> Self {
        self.jwt_config = Some(config);
        self
    }

    pub fn email_config(mut self, config: EmailConfig) -> Self {
        self.email_config = Some(config);
        self
    }

    pub fn pool(mut self, pool: Arc<PgPool>) -> Self {
        self.pool = Some(pool);
        self
    }

    pub fn build(self) -> Result<Config, String> {
        let server_config = self.server_config.ok_or("Server config is required")?;
        let jwt_config = self.jwt_config.ok_or("JWT config is required")?;
        let email_config = self.email_config.ok_or("Email config is required")?;
        let pool = self.pool.ok_or("Database pool is required")?;

        let user_repository = UserRepository::new(pool.clone());
        let email_verification_repository = EmailVerificationRepository::new(pool.clone());

        let password_service = PasswordService::new();
        let user_service = UserService::new(user_repository, password_service);
        let jwt_service = JwtService::new(jwt_config);

        let token_service = TokenService::new();

        let email_verification_service = EmailVerificationService::new(
            email_verification_repository,
            token_service,
            email_config.verification_token_expiration(),
        );

        let services = Services::new(user_service, jwt_service, email_verification_service);

        Ok(Config {
            server_config,
            services,
        })
    }
}
