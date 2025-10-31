//! Defines the business logic interface for managing users.

use async_trait::async_trait;
use crate::domain::user::User;

/// A trait that defines the business logic for managing users.
#[async_trait]
pub trait UserService {
    /// Creates a new user.
    async fn create_user(&self, name: &str, email: &str) -> anyhow::Result<User>;
    /// Lists all users.
    async fn list_users(&self) -> anyhow::Result<Vec<User>>;
}
