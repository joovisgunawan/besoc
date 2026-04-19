use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::config::Config;

pub async fn connect_primary_db(config: &Config) -> PgPool {
    PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.primary_url)
        .await
        .unwrap_or_else(|e| {
            tracing::error!("Failed to connect to primary database: {e}");
            std::process::exit(1);
        })
}

pub async fn connect_replica_db(config: &Config) -> PgPool {
    PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.replica_url)
        .await
        .unwrap_or_else(|e| {
            tracing::error!("Failed to connect to replica database: {e}");
            std::process::exit(1);
        })
}