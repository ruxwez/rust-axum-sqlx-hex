use std::sync::Arc;

use axum::{Json, http::StatusCode};

use crate::application::services::user::UserService;

pub async fn create_user() -> (StatusCode, Json<()>) {
    (StatusCode::OK, Json(()))
}
