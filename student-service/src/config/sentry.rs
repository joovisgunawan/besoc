use crate::config::helpers::{get_env, parse_env};
use std::env;

#[derive(Debug, Clone)]
pub struct SentryConfig {
    pub dsn: Option<String>,
    pub environment: String,
    pub traces_sample_rate: f32,
}

impl SentryConfig {
    pub fn from_env(invalid: &mut Vec<String>) -> Self {
        Self {
            dsn: env::var("SENTRY_DSN").ok(),
            environment: get_env("SENTRY_ENVIRONMENT", "development"),
            traces_sample_rate: parse_env("SENTRY_TRACES_SAMPLE_RATE", 1.0, invalid),
        }
    }
}