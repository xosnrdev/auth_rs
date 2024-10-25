use std::sync::Arc;

use crate::models::RefreshToken;
use chrono::Utc;
use sqlx::{Error, PgPool};
use uuid::Uuid;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct RefreshTokenRepository {
    pool: Arc<PgPool>,
}

impl RefreshTokenRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, token: &RefreshToken) -> Result<RefreshToken> {
        let token = sqlx::query_as!(
            RefreshToken,
            r#"
            INSERT INTO refresh_tokens (id, user_id, token, expires_at, revoked, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            token.id,
            token.user_id,
            token.token,
            token.expires_at,
            token.revoked,
            token.created_at,
            token.updated_at
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(token)
    }

    pub async fn get_by_user_id(&self, user_id: Uuid) -> Result<Option<RefreshToken>> {
        let token = sqlx::query_as!(
            RefreshToken,
            r#"
            SELECT * FROM refresh_tokens
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(token)
    }

    pub async fn get_by_token(&self, token: &str) -> Result<Option<RefreshToken>> {
        let token = sqlx::query_as!(
            RefreshToken,
            r#"
            SELECT * FROM refresh_tokens
            WHERE token = $1
            "#,
            token
        )
        .fetch_optional(&*self.pool)
        .await?;

        Ok(token)
    }

    pub async fn revoke(&self, user_id: Uuid) -> Result<RefreshToken> {
        let token = sqlx::query_as!(
            RefreshToken,
            r#"
            UPDATE refresh_tokens
            SET revoked = true, updated_at = $1
            WHERE user_id = $2
            RETURNING *
            "#,
            Utc::now(),
            user_id
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(token)
    }

    pub async fn delete(&self, user_id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM refresh_tokens
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }
}
