use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    api::handlers,
    application::services::{self, user::UserService},
    infrastructure::{
        database::postgres::PGPool,
        persistence::{self, user::UserRepository},
    },
};

pub struct StateRepositories {
    pub user: Arc<UserRepository>,
}

#[derive(Clone)]
pub struct StateServices {
    pub user: Arc<UserService>,
}

pub fn new(db_pool: PGPool) -> Router {
    // Init the repositories here
    let repos = Arc::new(StateRepositories {
        user: Arc::new(persistence::user::new_user_repository(db_pool.clone())),
    });

    // Init the services here
    let services = StateServices {
        user: Arc::new(services::user::new_user_service(repos.clone())),
    };

    // Init the routes here
    Router::new()
        .route("/api/user", post(handlers::user::create_user))
        .with_state(services)
}
