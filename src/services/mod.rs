mod email_verification;
mod user;

pub use email_verification::*;
pub use user::*;

pub struct Services {
    user_service: UserService,
    jwt_service: JwtService,
    email_verification_service: EmailVerificationService,
}

impl Services {
    pub fn new(
        user_service: UserService,
        jwt_service: JwtService,
        email_verification_service: EmailVerificationService,
    ) -> Self {
        Self {
            user_service,
            jwt_service,
            email_verification_service,
        }
    }

    pub fn user_service(&self) -> &UserService {
        &self.user_service
    }

    pub fn jwt_service(&self) -> &JwtService {
        &self.jwt_service
    }

    pub fn email_verification_service(&self) -> &EmailVerificationService {
        &self.email_verification_service
    }
}
