//! Implements the data access logic for managing users.

use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::domain::user::User;
use crate::domain::user_repository::UserRepository;

/// An implementation of the `UserRepository` trait.
pub struct UserRepositoryImpl {
    pool: SqlitePool,
}

impl UserRepositoryImpl {
    /// Creates a new `UserRepositoryImpl`.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    /// Creates a new user.
    async fn create_user(&self, name: &str, email: &str) -> anyhow::Result<User> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (name, email) VALUES (?, ?) RETURNING id, name, email",
            name,
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Lists all users.
    async fn list_users(&self) -> anyhow::Result<Vec<User>> {
        let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }
}
