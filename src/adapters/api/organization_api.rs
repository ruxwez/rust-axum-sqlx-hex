//! Defines the API endpoints for managing organizations.

use std::sync::Arc;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use crate::domain::organization_service::OrganizationService;
use crate::domain::organization::Organization;

/// The request body for creating an organization.
#[derive(Debug, Deserialize)]
pub struct CreateOrganization {
    pub name: String,
}

/// Creates the organization API routes.
pub fn create_organization_routes<T: OrganizationService + Send + Sync + 'static>() -> Router {
    Router::new()
        .route("/organizations", post(create_organization::<T>))
        .route("/organizations", get(list_organizations::<T>))
}

/// The handler for creating an organization.
async fn create_organization<T: OrganizationService>(
    Extension(organization_service): Extension<Arc<T>>,
    Json(payload): Json<CreateOrganization>,
) -> Result<Json<Organization>, AppError> {
    let organization = organization_service
        .create_organization(&payload.name)
        .await?;
    Ok(Json(organization))
}

/// The handler for listing organizations.
async fn list_organizations<T: OrganizationService>(
    Extension(organization_service): Extension<Arc<T>>,
) -> Result<Json<Vec<Organization>>, AppError> {
    let organizations = organization_service.list_organizations().await?;
    Ok(Json(organizations))
}

// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
