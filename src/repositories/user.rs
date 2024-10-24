use chrono::Utc;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::models::User;

type Result<T> = std::result::Result<T, Error>;
pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: &User) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            user.id,
            user.email,
            user.password_hash,
            user.created_at,
            user.updated_at
        )
        .fetch_one(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_all(&self, limit: i64, offset: i64) -> Result<Vec<User>> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            LIMIT $1
            OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(self.pool)
        .await?;
        Ok(users)
    }

    pub async fn update_email(&self, id: Uuid, email: &str) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET email = $1, updated_at = $2
            WHERE id = $3
            RETURNING *
            "#,
            email,
            Utc::now(),
            id
        )
        .fetch_one(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn update_password(&self, id: Uuid, password_hash: &str) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET password_hash = $1, updated_at = $2
            WHERE id = $3
            RETURNING *
            "#,
            password_hash,
            Utc::now(),
            id
        )
        .fetch_one(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }
}
