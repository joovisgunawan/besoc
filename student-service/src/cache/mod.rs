use redis::{aio::ConnectionManager, AsyncCommands, Client};
use serde::{de::DeserializeOwned, Serialize};
use tracing::instrument;

#[derive(Clone)]
pub struct RedisCache {
    conn: ConnectionManager,
}

impl RedisCache {
    pub async fn new(url: &str) -> Self {
        let client = Client::open(url).unwrap_or_else(|e| {
            tracing::error!("Failed to create Redis client: {e}");
            std::process::exit(1);
        });

        let conn = ConnectionManager::new(client).await.unwrap_or_else(|e| {
            tracing::error!("Failed to connect to Redis: {e}");
            std::process::exit(1);
        });

        Self { conn }
    }

    #[instrument(skip(self), name = "cache.get", fields(cache.key = %key))]
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let mut conn = self.conn.clone();
        let result: Option<String> = conn.get(key).await.unwrap_or_else(|e| {
            tracing::warn!(error = %e, "Redis GET failed");
            None
        });

        result.and_then(|s| {
            serde_json::from_str(&s).unwrap_or_else(|e| {
                tracing::warn!(error = %e, "Failed to deserialize cache value");
                None
            })
        })
    }

    #[instrument(skip(self, value), name = "cache.set", fields(cache.key = %key, cache.ttl = ttl_secs))]
    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl_secs: u64) {
        let mut conn = self.conn.clone();
        let Ok(serialized) = serde_json::to_string(value) else {
            tracing::warn!("Failed to serialize cache value");
            return;
        };

        if let Err(e) = conn.set_ex::<_, _, ()>(key, serialized, ttl_secs).await {
            tracing::warn!(error = %e, "Redis SET failed");
        }
    }

    #[instrument(skip(self), name = "cache.delete", fields(cache.key = %key))]
    pub async fn delete(&self, key: &str) {
        let mut conn = self.conn.clone();
        if let Err(e) = conn.del::<_, ()>(key).await {
            tracing::warn!(error = %e, "Redis DEL failed");
        }
    }

    #[instrument(skip(self), name = "cache.delete_pattern", fields(cache.pattern = %pattern))]
    pub async fn delete_pattern(&self, pattern: &str) {
        let mut conn = self.conn.clone();
        let keys: Vec<String> = conn.keys(pattern).await.unwrap_or_else(|e| {
            tracing::warn!(error = %e, "Redis KEYS failed");
            vec![]
        });

        if keys.is_empty() {
            return;
        }

        if let Err(e) = conn.del::<_, ()>(keys).await {
            tracing::warn!(error = %e, "Redis DEL pattern failed");
        }
    }

    pub async fn ping(&self) -> bool {
        let mut conn = self.conn.clone();
        redis::cmd("PING")
            .query_async::<_, String>(&mut conn)
            .await
            .is_ok()
    }
}

pub fn student_key(uuid: &str) -> String {
    format!("student:{uuid}")
}

pub fn students_list_key(page: i64, per_page: i64) -> String {
    format!("students:list:{page}:{per_page}")
}