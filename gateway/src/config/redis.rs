use std::env;
use sdk::constants::constant::{REDIS_HOST, REDIS_NAME, REDIS_PASSWORD, REDIS_PORT, REDIS_USERNAME};

#[derive(Debug, Clone)]
pub struct Redis {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub name: String,
}

impl Redis {
    pub fn new() -> Self {
        Self {
            host: env::var(REDIS_HOST).unwrap_or_default(),
            port: env::var(REDIS_PORT).unwrap_or_default(),
            username: env::var(REDIS_USERNAME).unwrap_or_default(),
            password: env::var(REDIS_PASSWORD).unwrap_or_default(),
            name: env::var(REDIS_NAME).unwrap_or_default(),
        }
    }
}
