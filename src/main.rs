//! The main entry point for the application.

use std::sync::Arc;
use axum::{Extension, Router};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod adapters;
mod application;
mod domain;
mod infrastructure;

use crate::{
    adapters::{
        api::{
            organization_api::create_organization_routes, user_api::create_user_routes,
        },
        repositories::{
            organization_repository::OrganizationRepositoryImpl,
            user_repository::UserRepositoryImpl,
        },
    },
    application::services::{
        organization_service::OrganizationServiceImpl, user_service::UserServiceImpl,
    },
    infrastructure::database::init_db,
};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_sqlite=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize the database
    let db_pool = init_db().await.unwrap();

    // Create the repositories
    let user_repository = Arc::new(UserRepositoryImpl::new(db_pool.clone()));
    let organization_repository =
        Arc::new(OrganizationRepositoryImpl::new(db_pool.clone()));

    // Create the services
    let user_service = Arc::new(UserServiceImpl::new(user_repository.clone()));
    let organization_service = Arc::new(OrganizationServiceImpl::new(
        organization_repository.clone(),
    ));

    // Create the Axum router
    let app = Router::new()
        .merge(create_user_routes::<UserServiceImpl<UserRepositoryImpl>>())
        .merge(create_organization_routes::<OrganizationServiceImpl<OrganizationRepositoryImpl>>())
        .layer(Extension(user_service))
        .layer(Extension(organization_service));

    // Start the server
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
