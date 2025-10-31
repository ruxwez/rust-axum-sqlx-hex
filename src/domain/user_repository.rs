//! Defines the data access interface for managing users.

use async_trait::async_trait;
use crate::domain::user::User;

/// A trait that defines the data access logic for managing users.
#[async_trait]
pub trait UserRepository {
    /// Creates a new user.
    async fn create_user(&self, name: &str, email: &str) -> anyhow::Result<User>;
    /// Lists all users.
    async fn list_users(&self) -> anyhow::Result<Vec<User>>;
}
