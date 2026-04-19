use crate::config::helpers::{parse_env, require_env};
use std::fmt;

#[derive(Clone)]
pub struct DatabaseConfig {
    pub primary_url: String,
    pub replica_url: String,
    pub max_connections: u32,
}

impl fmt::Debug for DatabaseConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatabaseConfig")
            .field("primary_url", &"[redacted]")
            .field("replica_url", &"[redacted]")
            .field("max_connections", &self.max_connections)
            .finish()
    }
}

impl DatabaseConfig {
    pub fn from_env(missing: &mut Vec<String>, invalid: &mut Vec<String>) -> Self {
        Self {
            primary_url: require_env("DATABASE_PRIMARY_URL", missing)
                .unwrap_or_default(),
            replica_url: require_env("DATABASE_REPLICA_URL", missing)
                .unwrap_or_default(),
            max_connections: parse_env("DB_MAX_CONNECTIONS", 10, invalid),
        }
    }
}