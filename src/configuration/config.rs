use super::{DbConfig, JwtConfig, ServerConfig};
use crate::repository::UserRepository;
use crate::service::{JwtService, Services, UserService};
use regex::Regex;
use sqlx::postgres::PgPoolOptions;

pub struct Config {
    pub server_config: ServerConfig,
    pub services: Services,
}

impl Config {
    pub async fn new(
        server_config: ServerConfig,
        db_config: DbConfig,
        jwt_config: JwtConfig,
    ) -> Self {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-.][a-z0-9]+)*\.[a-z]{2,6})",
        )
        .expect("Invalid email regex");

        let pool = PgPoolOptions::new().connect_lazy_with(db_config.to_pg_connect_options());

        let user_repository = UserRepository::new(pool, email_regex);

        let user_service = UserService::new(user_repository);
        let jwt_service = JwtService::new(jwt_config);

        let services = Services::new(user_service, jwt_service);

        Self {
            server_config,
            services,
        }
    }
}
