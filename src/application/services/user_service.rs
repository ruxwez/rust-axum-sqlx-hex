//! Implements the business logic for managing users.

use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::user::User;
use crate::domain::user_service::UserService;
use crate::domain::user_repository::UserRepository;

/// An implementation of the `UserService` trait.
pub struct UserServiceImpl<T: UserRepository> {
    user_repository: Arc<T>,
}

impl<T: UserRepository> UserServiceImpl<T> {
    /// Creates a new `UserServiceImpl`.
    pub fn new(user_repository: Arc<T>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl<T: UserRepository + Send + Sync> UserService for UserServiceImpl<T> {
    /// Creates a new user.
    async fn create_user(&self, name: &str, email: &str) -> anyhow::Result<User> {
        self.user_repository.create_user(name, email).await
    }

    /// Lists all users.
    async fn list_users(&self) -> anyhow::Result<Vec<User>> {
        self.user_repository.list_users().await
    }
}
