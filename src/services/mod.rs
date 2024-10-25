mod auth;
mod jwt;
mod refresh_token;
mod user;

pub use auth::*;
pub use jwt::*;
pub use refresh_token::*;
pub use user::*;

#[derive(Debug, Clone)]
pub struct Services {
    auth_service: AuthService,
}

impl Services {
    pub fn new(auth_service: AuthService) -> Self {
        Self { auth_service }
    }

    pub fn get_auth_service(&self) -> &AuthService {
        &self.auth_service
    }
}
