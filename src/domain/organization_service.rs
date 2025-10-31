//! Defines the business logic interface for managing organizations.

use async_trait::async_trait;
use crate::domain::organization::Organization;

/// A trait that defines the business logic for managing organizations.
#[async_trait]
pub trait OrganizationService {
    /// Creates a new organization.
    async fn create_organization(&self, name: &str) -> anyhow::Result<Organization>;
    /// Lists all organizations.
    async fn list_organizations(&self) -> anyhow::Result<Vec<Organization>>;
}
