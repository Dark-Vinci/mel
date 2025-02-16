use {
    sdk::constants::constant::{
        DB_HOST, DB_NAME, DB_PASSWORD, DB_PORT, DB_USERNAME,
    },
    std::env,
};

#[derive(Debug)]
pub struct DBConfig {
    pub password: String,
    pub username: String,
    pub host: String,
    pub port: String,
    pub name: String,
}

impl DBConfig {
    pub fn new() -> Self {
        Self {
            password: env::var(DB_PASSWORD).unwrap_or("melon".into()),
            username: env::var(DB_USERNAME).unwrap_or("melon".into()),
            host: env::var(DB_HOST).unwrap_or("localhost".into()),
            port: env::var(DB_PORT).unwrap_or("5420".into()),
            name: env::var(DB_NAME).unwrap_or("melon".into()),
        }
    }
}
