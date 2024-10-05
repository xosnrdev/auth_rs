mod jwt;
mod password;
mod user;
mod validation;

pub use jwt::*;
pub use password::*;
pub use user::*;
pub use validation::*;

pub struct Services {
    pub user_service: UserService,
    pub jwt_service: JwtService,
}

impl Services {
    pub fn new(user_service: UserService, jwt_service: JwtService) -> Self {
        Self {
            user_service,
            jwt_service,
        }
    }
}
