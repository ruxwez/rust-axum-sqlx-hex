//! Implements the business logic for managing organizations.

use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::organization::Organization;
use crate::domain::organization_service::OrganizationService;
use crate::domain::organization_repository::OrganizationRepository;

/// An implementation of the `OrganizationService` trait.
pub struct OrganizationServiceImpl<T: OrganizationRepository> {
    organization_repository: Arc<T>,
}

impl<T: OrganizationRepository> OrganizationServiceImpl<T> {
    /// Creates a new `OrganizationServiceImpl`.
    pub fn new(organization_repository: Arc<T>) -> Self {
        Self { organization_repository }
    }
}

#[async_trait]
impl<T: OrganizationRepository + Send + Sync> OrganizationService for OrganizationServiceImpl<T> {
    /// Creates a new organization.
    async fn create_organization(&self, name: &str) -> anyhow::Result<Organization> {
        self.organization_repository.create_organization(name).await
    }

    /// Lists all organizations.
    async fn list_organizations(&self) -> anyhow::Result<Vec<Organization>> {
        self.organization_repository.list_organizations().await
    }
}
