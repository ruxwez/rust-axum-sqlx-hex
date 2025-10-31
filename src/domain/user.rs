//! Represents a user in the domain.
use serde::Serialize;
use sqlx::FromRow;

/// Represents a user entity.
#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}
