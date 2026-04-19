use crate::config::helpers::{get_env, parse_env};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            app_name: get_env("APP_NAME", "student-service"),
            host: get_env("APP_HOST", "0.0.0.0"),
            port: parse_env("APP_PORT", 8080, &mut vec![]),
        }
    }
}