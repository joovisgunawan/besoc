use std::env;

pub fn get_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

pub fn require_env(key: &str, missing: &mut Vec<String>) -> Option<String> {
    match env::var(key) {
        Ok(v) => Some(v),
        Err(_) => {
            missing.push(key.to_string());
            None
        }
    }
}

pub fn parse_env<T>(key: &str, default: T, invalid: &mut Vec<String>) -> T
where
    T: std::str::FromStr + Copy,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    match env::var(key) {
        Ok(v) => match v.parse::<T>() {
            Ok(p) => p,
            Err(_) => {
                invalid.push(format!("{}={:?}", key, v));
                default
            }
        },
        Err(_) => default,
    }
}