use crate::models::User;
use chrono::{DateTime, Utc};
use regex::Regex;
use sqlx::{Error as SQLxError, PgPool};
use std::time::SystemTime;
use uuid::Uuid;

pub struct UserRepository {
    pool: PgPool,
    pub email_regex: Regex,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Empty username")]
    EmptyUsername,
    #[error("Empty table")]
    EmptyTable,
    #[error("Empty email")]
    EmptyEmail,
    #[error("Empty password")]
    EmptyPassword,
    #[error("Empty text search")]
    EmptyTextSearch,
    #[error("User not found: {0}")]
    UserNotFound(String),
    #[error("Username already taken")]
    UsernameAlreadyTaken,
    #[error("Email already taken")]
    EmailAlreadyTaken,
    #[error("Invalid email address: {0}")]
    InvalidEmail(String),
    #[error("SQLx error: {0}")]
    SQLx(#[from] SQLxError),
}

impl UserRepository {
    pub fn new(pool: PgPool, email_regex: Regex) -> Self {
        UserRepository { pool, email_regex }
    }

    pub async fn create(&self, user: User) -> Result<User, Error> {
        self.validate_user(&user)?;

        if self.exists_username(&user.username).await? {
            return Err(Error::UsernameAlreadyTaken);
        }

        if let Some(ref email) = user.email {
            if self.exists_email(email).await? {
                return Err(Error::EmailAlreadyTaken);
            }
        }

        let result = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            user.username,
            user.email,
            user.password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    fn validate_user(&self, user: &User) -> Result<(), Error> {
        if user.username.is_empty() {
            return Err(Error::EmptyUsername);
        }

        match user.email {
            Some(ref email) => {
                if email.is_empty() {
                    return Err(Error::EmptyEmail);
                }
                if !self.email_regex.is_match(email) {
                    return Err(Error::InvalidEmail(email.to_string()));
                }
            }
            None => return Err(Error::EmptyEmail),
        }

        if user.password.is_empty() {
            return Err(Error::EmptyPassword);
        }
        Ok(())
    }

    async fn exists_username(&self, username: &str) -> Result<bool, Error> {
        let user = self.find_by_username(username).await?;
        Ok(user.is_some())
    }

    async fn exists_email(&self, email: &str) -> Result<bool, Error> {
        let user = self.find_by_email(email).await?;
        Ok(user.is_some())
    }

    pub async fn find_all(
        &self,
        limit: Option<i64>,
        page: Option<i64>,
    ) -> Result<Vec<User>, Error> {
        let limit = limit.unwrap_or(10);
        let offset = page.unwrap_or(0) * limit;

        let users = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Error::SQLx)?;

        if users.is_empty() {
            return Err(Error::EmptyTable);
        }

        Ok(users)
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(Error::SQLx)?;

        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        if username.is_empty() {
            return Err(Error::EmptyUsername);
        }

        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE LOWER(username) = LOWER($1)",
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::SQLx)?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        if email.is_empty() {
            return Err(Error::EmptyEmail);
        }

        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE LOWER(email) = LOWER($1)",
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::SQLx)?;

        Ok(user)
    }

    pub async fn update(&self, user: &User) -> Result<User, Error> {
        match self.find_by_username(&user.username).await {
            Ok(u) => {
                if let Some(existing_user) = u {
                    if existing_user.id != user.id {
                        return Err(Error::UsernameAlreadyTaken);
                    }
                }
            }
            Err(e) => return Err(e),
        }

        if user.email.is_some() {
            match self.find_by_email(user.email.as_ref().unwrap()).await {
                Ok(u) => {
                    if let Some(existing_user) = u {
                        if existing_user.id != user.id {
                            return Err(Error::EmailAlreadyTaken);
                        }
                    }
                }
                Err(e) => return Err(e),
            }
        }

        let now: DateTime<Utc> = SystemTime::now().into();

        let result = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET username = $1, email = $2, updated_at = $3
            WHERE id = $4
            RETURNING *
            "#,
            user.username,
            user.email,
            now,
            user.id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(Error::SQLx)?;

        Ok(result)
    }

    pub async fn update_password(&self, id: &Uuid, new_password: &str) -> Result<(), Error> {
        if new_password.is_empty() {
            return Err(Error::EmptyPassword);
        }

        let result = sqlx::query!(
            r#"
            UPDATE users
            SET password = $1
            WHERE id = $2
            "#,
            new_password,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(Error::SQLx)?;

        if result.rows_affected() == 0 {
            return Err(Error::UserNotFound(id.to_string()));
        }

        Ok(())
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), Error> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(Error::SQLx)?;

        if result.rows_affected() == 0 {
            return Err(Error::UserNotFound(id.to_string()));
        }

        Ok(())
    }

    pub async fn search(&self, query: &str) -> Result<Vec<User>, Error> {
        if query.is_empty() {
            return Err(Error::EmptyTextSearch);
        }

        let users = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE username ILIKE $1 OR email ILIKE $1
            "#,
            format!("%{}%", query)
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Error::SQLx)?;

        if users.is_empty() {
            return Err(Error::EmptyTable);
        }

        Ok(users)
    }
}
