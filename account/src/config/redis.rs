use {
    sdk::constants::constant::{
        REDIS_HOST, REDIS_PASSWORD, REDIS_PORT, REDIS_USERNAME,
    },
    std::env,
};

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub password: String,
    pub username: String,
    pub host: String,
    pub port: String,
}

impl RedisConfig {
    pub fn new() -> Self {
        Self {
            password: env::var(REDIS_PASSWORD).unwrap_or_default(),
            username: env::var(REDIS_USERNAME).unwrap_or_default(),
            host: env::var(REDIS_HOST).unwrap_or_default(),
            port: env::var(REDIS_PORT).unwrap_or_default(),
        }
    }
}
