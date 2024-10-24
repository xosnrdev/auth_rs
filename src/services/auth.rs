use uuid::Uuid;

use crate::{
    dto::{AuthResponse, AuthStatus, TokenDetails},
    models::{Credentials, RefreshToken, User},
    services::UserServiceError,
    utils::{AuthServiceError, Hashing, JWT},
};

use super::{RefreshTokenService, UserService};

type Result<T> = std::result::Result<T, AuthServiceError>;

pub struct AuthService<'a> {
    user_service: UserService<'a>,
    refresh_token_service: RefreshTokenService<'a>,
    jwt: JWT,
}

impl<'a> AuthService<'a> {
    pub fn new(
        user_service: UserService<'a>,
        refresh_token_service: RefreshTokenService<'a>,
        jwt: JWT,
    ) -> Self {
        Self {
            user_service,
            refresh_token_service,
            jwt,
        }
    }

    async fn store_token(&self, user_id: Uuid, refresh_token: &str) -> Result<RefreshToken> {
        let token = self
            .refresh_token_service
            .create(user_id, refresh_token)
            .await
            .map_err(AuthServiceError::from)?;

        Ok(token)
    }

    async fn generate_tokens(&self, user_id: Uuid, email: &str) -> Result<TokenDetails> {
        let mut refresh_token = self
            .jwt
            .generate_refresh_token(user_id, &email)
            .map_err(AuthServiceError::from)?;

        let token_details = self.refresh_access_token(&refresh_token).await?;

        let access_token = token_details.get_token();

        let expires_in = self.jwt.get_expires_in(access_token)?;

        let x_refresh_token = self
            .refresh_token_service
            .get_by_user_id(user_id)
            .await
            .map_err(AuthServiceError::from)?;

        let stored_refresh_token = match x_refresh_token {
            Some(r_token) => r_token,
            None => {
                self.store_token(user_id, &refresh_token).await?;
                return Ok(token_details);
            }
        };

        if stored_refresh_token.is_expired() || stored_refresh_token.is_revoked() {
            self.refresh_token_service
                .delete(user_id)
                .await
                .map_err(AuthServiceError::from)?;

            self.store_token(user_id, &refresh_token).await?;
        } else {
            refresh_token = stored_refresh_token.token;
        }

        Ok(TokenDetails::new(access_token, expires_in, refresh_token))
    }

    pub async fn register(&self, cred: Credentials) -> Result<AuthResponse> {
        let x_user = self.user_service.get_by_email(&cred.email).await?;

        if x_user.is_some() {
            return Err(AuthServiceError::UserServiceError(
                UserServiceError::UserAlreadyExists,
            ));
        }

        let password_hash = Hashing::hash(&cred.password)?;

        let user = self
            .user_service
            .create(&cred.email, &password_hash)
            .await
            .map_err(AuthServiceError::from)?;

        let token_details = self.generate_tokens(user.id, &user.email).await?;

        Ok(AuthResponse::success(
            token_details,
            "Registration successful",
        ))
    }

    pub async fn authenticate(&self, cred: Credentials) -> Result<AuthResponse> {
        let x_user = self.user_service.get_by_email(&cred.email).await?;

        let user = match x_user {
            Some(user) => user,
            None => return Err(AuthServiceError::InvalidCredentials),
        };

        if !Hashing::verify(&cred.password, &user.password_hash)? {
            return Err(AuthServiceError::InvalidCredentials);
        }

        let token_details = self.generate_tokens(user.id, &user.email).await?;

        Ok(AuthResponse::success(
            token_details,
            "Authentication successful",
        ))
    }

    pub async fn refresh_access_token(&self, refresh_token: &str) -> Result<TokenDetails> {
        let x_token = self.validate_refresh_token(refresh_token).await?;

        match x_token {
            TokenValidity::Invalid => return Err(AuthServiceError::InvalidToken),
            TokenValidity::Valid(r_token) => {
                let token_data = self.jwt.decode(&r_token.token)?;

                let access_token = self
                    .jwt
                    .generate_access_token(
                        token_data.claims.get_user_id(),
                        &token_data.claims.get_email(),
                    )
                    .map_err(AuthServiceError::from)?;

                let expires_in = self.jwt.get_expires_in(&access_token)?;

                let token_details = TokenDetails::new(&access_token, expires_in, None);

                Ok(token_details)
            }
        }
    }

    async fn validate_refresh_token(&self, refresh_token: &str) -> Result<TokenValidity> {
        let x_token = self
            .refresh_token_service
            .get_by_token(refresh_token)
            .await?;

        match x_token {
            None => return Ok(TokenValidity::Invalid),
            Some(r_token) => {
                if r_token.is_expired()
                    || r_token.is_revoked()
                    || self.jwt.decode(refresh_token).is_err()
                {
                    self.refresh_token_service.delete(r_token.user_id).await?;
                    return Ok(TokenValidity::Invalid);
                }
                Ok(TokenValidity::Valid(r_token))
            }
        }
    }

    pub async fn logout(&self, refresh_token: &str) -> Result<AuthResponse> {
        let x_token = self.validate_refresh_token(refresh_token).await?;

        match x_token {
            TokenValidity::Invalid => return Err(AuthServiceError::InvalidToken),
            TokenValidity::Valid(r_token) => {
                self.refresh_token_service.delete(r_token.user_id).await?;
                Ok(AuthResponse::new(AuthStatus::Success, "Logout successful"))
            }
        }
    }

    pub async fn delete_me(&self, access_token: &str) -> Result<AuthResponse> {
        let x_token = self.jwt.decode(access_token)?;

        let user_id = x_token.claims.get_user_id();

        self.refresh_token_service.delete(user_id).await?;

        self.user_service.delete(user_id).await?;

        Ok(AuthResponse::new(
            AuthStatus::Success,
            "Deletion successful",
        ))
    }

    pub async fn get_me(&self, refresh_token: &str) -> Result<User> {
        let x_token = self.validate_refresh_token(refresh_token).await?;

        match x_token {
            TokenValidity::Invalid => return Err(AuthServiceError::InvalidToken),
            TokenValidity::Valid(r_token) => {
                let user = self.user_service.get_by_id(r_token.user_id).await?;

                match user {
                    Some(user) => Ok(user),
                    None => Err(AuthServiceError::UserServiceError(
                        UserServiceError::UserNotFound,
                    )),
                }
            }
        }
    }

    // TODO: Implement pagination
    // TODO: Must require admin, or moderators authorization which is not supported for now maybe in the future
    pub async fn get_all_users(&self, _refresh_token: &str) -> Result<Vec<User>> {
        todo!()
    }

    pub async fn update_email(&self, access_token: &str, new_email: &str) -> Result<AuthResponse> {
        let x_token = self.jwt.decode(access_token)?;

        let user_id = x_token.claims.get_user_id();

        let x_user = self.user_service.get_by_id(user_id).await?;

        match x_user {
            Some(user) => {
                self.user_service.update_email(user.id, new_email).await?;

                // Invalidate all sessions
                self.refresh_token_service.delete(user.id).await?;

                Ok(AuthResponse::new(
                    AuthStatus::Success,
                    "Email update successful",
                ))
            }
            None => Err(AuthServiceError::UserServiceError(
                UserServiceError::UserNotFound,
            )),
        }
    }

    pub async fn update_password(
        &self,
        access_token: &str,
        new_password: &str,
    ) -> Result<AuthResponse> {
        let x_token = self.jwt.decode(access_token)?;

        let user_id = x_token.claims.get_user_id();

        let x_user = self.user_service.get_by_id(user_id).await?;

        match x_user {
            Some(user) => {
                let password_hash = Hashing::hash(new_password)?;

                self.user_service
                    .update_password(user.id, &password_hash)
                    .await?;

                // Invalidate all sessions
                self.refresh_token_service.delete(user.id).await?;

                Ok(AuthResponse::new(
                    AuthStatus::Success,
                    "Password update successful",
                ))
            }
            None => Err(AuthServiceError::UserServiceError(
                UserServiceError::UserNotFound,
            )),
        }
    }

    // TODO: Admin, or moderators authorization required
    pub async fn revoke_user_refresh_token(
        &self,
        _access_token: &str,
        _user_id: Uuid,
    ) -> Result<AuthResponse> {
        todo!()
    }

    // TODO: Admin, or moderators authorization required
    pub async fn delete_user(&self, _access_token: &str, _user_id: Uuid) -> Result<AuthResponse> {
        todo!()
    }
}

enum TokenValidity {
    Valid(RefreshToken),
    Invalid,
}
