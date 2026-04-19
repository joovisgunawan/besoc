mod app;
mod auth;
mod database;
mod helpers;
mod kafka;
mod redis;
mod sentry;
mod telemetry;

pub use app::AppConfig;
pub use auth::AuthConfig;
pub use database::DatabaseConfig;
pub use kafka::KafkaConfig;
pub use redis::RedisConfig;
pub use sentry::SentryConfig;
pub use telemetry::TelemetryConfig;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Config {
    pub app: AppConfig,
    pub auth: AuthConfig,
    pub database: DatabaseConfig,
    pub kafka: KafkaConfig,
    pub redis: RedisConfig,
    pub sentry: SentryConfig,
    pub telemetry: TelemetryConfig,
}

#[derive(Debug)]
pub struct ConfigError {
    pub missing: Vec<String>,
    pub invalid: Vec<String>,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.missing.is_empty() {
            writeln!(f, "Missing required env vars: {}", self.missing.join(", "))?;
        }
        if !self.invalid.is_empty() {
            writeln!(f, "Invalid env var values: {}", self.invalid.join(", "))?;
        }
        Ok(())
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut missing = Vec::new();
        let mut invalid = Vec::new();

        let config = Self {
            app: AppConfig::from_env(),
            auth: AuthConfig::from_env(&mut missing, &mut invalid),
            database: DatabaseConfig::from_env(&mut missing, &mut invalid),
            kafka: KafkaConfig::from_env(),
            redis: RedisConfig::from_env(&mut missing),
            sentry: SentryConfig::from_env(&mut invalid),
            telemetry: TelemetryConfig::from_env(&mut invalid),
        };

        if !missing.is_empty() || !invalid.is_empty() {
            return Err(ConfigError { missing, invalid });
        }

        Ok(config)
    }
}