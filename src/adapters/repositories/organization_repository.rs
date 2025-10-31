//! Implements the data access logic for managing organizations.

use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::domain::organization::Organization;
use crate::domain::organization_repository::OrganizationRepository;

/// An implementation of the `OrganizationRepository` trait.
pub struct OrganizationRepositoryImpl {
    pool: SqlitePool,
}

impl OrganizationRepositoryImpl {
    /// Creates a new `OrganizationRepositoryImpl`.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrganizationRepository for OrganizationRepositoryImpl {
    /// Creates a new organization.
    async fn create_organization(&self, name: &str) -> anyhow::Result<Organization> {
        let organization = sqlx::query_as!(
            Organization,
            "INSERT INTO organizations (name) VALUES (?) RETURNING id, name",
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(organization)
    }

    /// Lists all organizations.
    async fn list_organizations(&self) -> anyhow::Result<Vec<Organization>> {
        let organizations = sqlx::query_as!(Organization, "SELECT id, name FROM organizations")
            .fetch_all(&self.pool)
            .await?;

        Ok(organizations)
    }
}
