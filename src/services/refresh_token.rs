use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

use crate::{models::RefreshToken, repositories::RefreshTokenRepository};

#[derive(Debug, Error)]
pub enum RefreshTokenServiceError {
    #[error("Refresh token not found")]
    RefreshTokenNotFound,
    #[error("Refresh token already exists")]
    RefreshTokenAlreadyExists,
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
}

type Result<T> = std::result::Result<T, RefreshTokenServiceError>;

#[derive(Debug, Clone)]
pub struct RefreshTokenService {
    repository: RefreshTokenRepository,
    refresh_token_duration_days: DateTime<Utc>,
}

impl RefreshTokenService {
    pub fn new(
        repository: RefreshTokenRepository,
        refresh_token_duration_days: DateTime<Utc>,
    ) -> Self {
        Self {
            repository,
            refresh_token_duration_days,
        }
    }

    pub async fn create(&self, user_id: Uuid, token: &str) -> Result<RefreshToken> {
        let x_token = self.repository.get_by_user_id(user_id).await?;

        if x_token.is_some() {
            return Err(RefreshTokenServiceError::RefreshTokenAlreadyExists);
        }

        let r_token = RefreshToken::new(user_id, token, self.refresh_token_duration_days);

        let new_token = self.repository.create(&r_token).await?;

        Ok(new_token)
    }

    pub async fn get_by_user_id(&self, user_id: Uuid) -> Result<Option<RefreshToken>> {
        let token = self.repository.get_by_user_id(user_id).await?;

        Ok(token)
    }

    pub async fn get_by_token(&self, token: &str) -> Result<Option<RefreshToken>> {
        let token = self.repository.get_by_token(token).await?;

        Ok(token)
    }

    pub async fn revoke(&self, user_id: Uuid) -> Result<RefreshToken> {
        let token = self.repository.revoke(user_id).await?;

        Ok(token)
    }

    pub async fn delete(&self, user_id: Uuid) -> Result<()> {
        self.repository.delete(user_id).await?;

        Ok(())
    }
}
