use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::{
    api::{
        dtos::{Response, user::CreateUserPayload},
        factory::StateServices,
    },
    domain::entities,
};

pub async fn create_user(
    State(services): State<Arc<StateServices>>,
    Json(payload): Json<CreateUserPayload>,
) -> (StatusCode, Json<Response<entities::User>>) {
    let user = match services
        .user
        .create_user(
            &payload.email,
            &payload.phone,
            &payload.first_name,
            &payload.last_name,
        )
        .await
    {
        Some(user) => user,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response {
                    success: false,
                    message: Some("Esto es una prueba".to_string()),
                    result: None,
                }),
            );
        }
    };

    (
        StatusCode::OK,
        Json(Response {
            success: true,
            message: None,
            result: Some(user),
        }),
    )
}
