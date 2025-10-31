//! Defines the data access interface for managing organizations.

use async_trait::async_trait;
use crate::domain::organization::Organization;

/// A trait that defines the data access logic for managing organizations.
#[async_trait]
pub trait OrganizationRepository {
    /// Creates a new organization.
    async fn create_organization(&self, name: &str) -> anyhow::Result<Organization>;
    /// Lists all organizations.
    async fn list_organizations(&self) -> anyhow::Result<Vec<Organization>>;
}
