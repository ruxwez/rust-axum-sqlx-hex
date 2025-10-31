//! Handles the database connection and initialization.

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

/// Initializes the database and returns a connection pool.
pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
