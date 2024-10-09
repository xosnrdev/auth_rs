use crate::models::EmailVerification;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct EmailVerificationRepository {
    pool: Arc<sqlx::PgPool>,
}

impl EmailVerificationRepository {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        verification: &EmailVerification,
    ) -> Result<EmailVerification, sqlx::Error> {
        sqlx::query_as!(
            EmailVerification,
            r#"
            INSERT INTO email_verification (id, user_id, token_hash, expires_at, created_at, updated_at, is_used)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            verification.id,
            verification.user_id,
            verification.token_hash,
            verification.expires_at,
            verification.created_at,
            verification.updated_at,
            verification.is_used
        )
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn get_all(&self) -> Result<Vec<EmailVerification>, sqlx::Error> {
        sqlx::query_as!(
            EmailVerification,
            "SELECT * FROM email_verification ORDER BY created_at DESC"
        )
        .fetch_all(&*self.pool)
        .await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<EmailVerification>, sqlx::Error> {
        sqlx::query_as!(
            EmailVerification,
            "SELECT * FROM email_verification WHERE id = $1",
            id
        )
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn get_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<EmailVerification>, sqlx::Error> {
        sqlx::query_as!(
            EmailVerification,
            "SELECT * FROM email_verification WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM email_verification WHERE id = $1", id)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_is_used(
        &self,
        id: Uuid,
        is_used: bool,
        updated_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE email_verification SET is_used = $1, updated_at = $2 WHERE id = $3",
            is_used,
            updated_at,
            id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_is_email_verified(
        &self,
        user_id: Uuid,
        is_email_verified: bool,
        updated_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET is_email_verified = $1, updated_at = $2 WHERE id = $3",
            is_email_verified,
            updated_at,
            user_id
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_by_token_hash(
        &self,
        token_hash: String,
    ) -> Result<Option<EmailVerification>, sqlx::Error> {
        sqlx::query_as!(
            EmailVerification,
            "SELECT * FROM email_verification WHERE token_hash = $1",
            token_hash
        )
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn get_expired_verifications(&self) -> Result<Vec<EmailVerification>, sqlx::Error> {
        sqlx::query_as!(
            EmailVerification,
            "SELECT * FROM email_verification WHERE expires_at < NOW() AND is_used = false"
        )
        .fetch_all(&*self.pool)
        .await
    }

    pub async fn delete_expired_verifications(&self) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM email_verification WHERE expires_at < NOW() AND is_used = false")
            .execute(&*self.pool)
            .await?;

        Ok(())
    }
}
