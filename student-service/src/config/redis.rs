use crate::config::helpers::require_env;
use std::fmt;

#[derive(Clone)]
pub struct RedisConfig {
    pub url: String,
}

impl fmt::Debug for RedisConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RedisConfig")
            .field("url", &"[redacted]")
            .finish()
    }
}

impl RedisConfig {
    pub fn from_env(missing: &mut Vec<String>) -> Self {
        Self {
            url: require_env("REDIS_URL", missing).unwrap_or_default(),
        }
    }
}