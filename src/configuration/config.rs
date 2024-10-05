use super::{DbConfig, JwtConfig, ServerConfig};
use crate::repository::UserRepository;
use crate::service::{JwtService, Services, UserService};
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
        let pool = PgPoolOptions::new().connect_lazy_with(db_config.to_pg_connect_options());

        let user_repository = UserRepository::new(pool);

        let user_service = UserService::new(user_repository);
        let jwt_service = JwtService::new(jwt_config);

        let services = Services::new(user_service, jwt_service);

        Self {
            server_config,
            services,
        }
    }
}
