use clap::Parser;

use crate::{api::factory, config::Config, infrastructure::database::postgres};

mod api;
mod application;
mod config;
mod infrastructure;

#[tokio::main]
async fn main() {
    // Instance dotenv to load environment variables from a .env file
    dotenvy::dotenv().ok();

    // Parse configuration
    let cfg = Config::parse();
    let db = postgres::new_postgres_pool(&cfg.database_url).await;

    let app = factory::new(db.clone());

    // Listen on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
