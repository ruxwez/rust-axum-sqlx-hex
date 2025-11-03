use std::sync::Arc;

use axum::{Json, http::StatusCode};

use crate::application::services::user::UserService;

pub struct UserHandlers {
    user_service: Arc<UserService>,
}

pub fn new_user_handlers(user_service: Arc<UserService>) -> UserHandlers {
    UserHandlers { user_service }
}

impl UserHandlers {
    pub async fn create_user(&self) -> (StatusCode, Json<()>) {
        (StatusCode::OK, Json(()))
    }
}
