use crate::config::helpers::{get_env, parse_env};

#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    pub otlp_endpoint: String,
    pub service_name: String,
    pub service_version: String,
    pub enabled: bool,
}

impl TelemetryConfig {
    pub fn from_env(invalid: &mut Vec<String>) -> Self {
        Self {
            otlp_endpoint: get_env("OTEL_EXPORTER_OTLP_ENDPOINT", "http://localhost:4317"),
            service_name: get_env("OTEL_SERVICE_NAME", "student-service"),
            service_version: get_env("OTEL_SERVICE_VERSION", "0.1.0"),
            enabled: parse_env("OTEL_ENABLED", true, invalid),
        }
    }
}