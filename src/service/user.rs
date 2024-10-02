use log::info;

use crate::{
    models::User,
    repository::{Error, UserRepository},
};

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        UserService { user_repository }
    }

    pub async fn create(&self, user: User) -> Result<User, Error> {
        info!("Creating user: {}", user);

        self.user_repository.create(user).await
    }

    pub async fn find_all(
        &self,
        limit: Option<i64>,
        page: Option<i64>,
    ) -> Result<Vec<User>, Error> {
        info!("Finding all users");

        self.user_repository.find_all(limit, page).await
    }

    pub async fn find_by_id(&self, id: &uuid::Uuid) -> Result<Option<User>, Error> {
        info!("Finding user by id: {}", id);

        self.user_repository.find_by_id(id).await
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        info!("Finding user by username: {}", username);

        self.user_repository.find_by_username(username).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        info!("Finding user by email: {}", email);

        self.user_repository.find_by_email(email).await
    }

    pub async fn update(&self, user: &User) -> Result<User, Error> {
        info!("Updating user: {}", user);

        self.user_repository.update(user).await
    }

    pub async fn update_password(&self, id: &uuid::Uuid, password: &str) -> Result<(), Error> {
        info!("Updating user password by id: {}", id);

        self.user_repository.update_password(id, password).await
    }

    pub async fn delete(&self, id: &uuid::Uuid) -> Result<(), Error> {
        info!("Deleting user by id: {}", id);

        self.user_repository.delete(id).await
    }

    pub async fn search(&self, query: &str) -> Result<Vec<User>, Error> {
        info!("Searching for users by query: {}", query);

        self.user_repository.search(query).await
    }

    pub async fn verify_email(&self, id: &uuid::Uuid) -> Result<(), Error> {
        info!("Verifying user email by id: {}", id);

        self.user_repository.verify_email(id).await
    }
}
