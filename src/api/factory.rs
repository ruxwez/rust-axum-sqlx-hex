use std::sync::Arc;

use axum::Router;
use sqlx::{Pool, Postgres};

use crate::{
    api::handlers,
    application::services,
    infrastructure::{database::postgres::PGPool, persistence},
};

pub fn new(db_pool: PGPool) -> Router {
    // Init the repositories here
    let user_repo = Arc::new(persistence::user::new_user_repository(db_pool.clone()));

    // Init the services here
    let user_service = Arc::new(services::user::new_user_service(user_repo.clone()));

    // Init the controllers/handlers here
    let user_handlers = handlers::user::new_user_handlers(user_service.clone());

    // Init the routes here
    Router::new()
}
