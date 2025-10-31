//! Represents an organization in the domain.
use serde::Serialize;
use sqlx::FromRow;

/// Represents an organization entity.
#[derive(Debug, Serialize, FromRow)]
pub struct Organization {
    pub id: i64,
    pub name: String,
}
