use sqlx::{Pool, Postgres, pool::PoolOptions};

// Not is necesary put the ARC because Pool already use it internally
pub type PGPool = Pool<Postgres>;

pub async fn new_postgres_pool(dsn: &str) -> PGPool {
    PoolOptions::new()
        .connect(dsn)
        .await
        .expect("Error al conectar con la base de datos Postgres")
}
