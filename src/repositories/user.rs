use crate::models::User;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: Arc<sqlx::PgPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, email, password_hash, is_email_verified,  
                               created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING * 
            "#,
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.is_email_verified,
            user.created_at,
            user.updated_at
        )
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn get_by_id(&self, id: &Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, r#"SELECT * FROM users WHERE id = $1"#, id)
            .fetch_optional(&*self.pool)
            .await
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, r#"SELECT * FROM users WHERE email ILIKE $1"#, email)
            .fetch_optional(&*self.pool)
            .await
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT * FROM users WHERE username ILIKE $1"#,
            username
        )
        .fetch_optional(&*self.pool)
        .await
    }

    pub async fn get_all(&self, limit: i64, offset: i64) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&*self.pool)
        .await
    }

    pub async fn update(
        &self,
        user: &User,
        updated_at: DateTime<Utc>,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET username = $2, email = $3, password_hash = $4, is_email_verified = $5,
                updated_at = $6
            WHERE id = $1
            RETURNING *
            "#,
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.is_email_verified,
            updated_at
        )
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn update_password(
        &self,
        id: &Uuid,
        password_hash: &str,
        updated_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET password_hash = $2, updated_at = $3
            WHERE id = $1
            "#,
            id,
            password_hash,
            updated_at
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_email(
        &self,
        id: &Uuid,
        email: &str,
        updated_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET email = $2, is_email_verified = NULL, updated_at = $3
            WHERE id = $1
            "#,
            id,
            email,
            updated_at
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }

    pub async fn search(
        &self,
        query: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<User>, sqlx::Error> {
        let search_query = format!("%{}%", query);
        sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE username ILIKE $1 OR email ILIKE $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            search_query,
            limit,
            offset
        )
        .fetch_all(&*self.pool)
        .await
    }

    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(r#"SELECT COUNT(*) as count FROM users"#)
            .fetch_one(&*self.pool)
            .await?;
        Ok(result.count.unwrap_or(0))
    }

    pub async fn count_search(&self, query: &str) -> Result<i64, sqlx::Error> {
        let search_query = format!("%{}%", query);
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as count FROM users
            WHERE username ILIKE $1 OR email ILIKE $1
            "#,
            search_query
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(result.count.unwrap_or(0))
    }
}
