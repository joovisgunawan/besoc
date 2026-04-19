use crate::config::helpers::{get_env, require_env};
use std::fmt;

#[derive(Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiry_minutes: u64,
}

impl fmt::Debug for AuthConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthConfig")
            .field("jwt_secret", &"[redacted]")
            .field("jwt_expiry_minutes", &self.jwt_expiry_minutes)
            .finish()
    }
}

impl AuthConfig {
    pub fn from_env(missing: &mut Vec<String>, invalid: &mut Vec<String>) -> Self {
        Self {
            jwt_secret: require_env("JWT_SECRET", missing).unwrap_or_default(),
            jwt_expiry_minutes: crate::config::helpers::parse_env(
                "JWT_EXPIRY_MINUTES",
                60,
                invalid,
            ),
        }
    }
}