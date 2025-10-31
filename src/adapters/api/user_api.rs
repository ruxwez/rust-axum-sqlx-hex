//! Defines the API endpoints for managing users.

use std::sync::Arc;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use crate::domain::user_service::UserService;
use crate::domain::user::User;

/// The request body for creating a user.
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

/// Creates the user API routes.
pub fn create_user_routes<T: UserService + Send + Sync + 'static>() -> Router {
    Router::new()
        .route("/users", post(create_user::<T>))
        .route("/users", get(list_users::<T>))
}

/// The handler for creating a user.
async fn create_user<T: UserService>(
    Extension(user_service): Extension<Arc<T>>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, AppError> {
    let user = user_service
        .create_user(&payload.name, &payload.email)
        .await?;
    Ok(Json(user))
}

/// The handler for listing users.
async fn list_users<T: UserService>(
    Extension(user_service): Extension<Arc<T>>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = user_service.list_users().await?;
    Ok(Json(users))
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
