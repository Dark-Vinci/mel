use {
    sdk::constants::constant::{ACCOUNT, ACCOUNT_PORT},
    std::env,
    uuid::Uuid,
};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub service_name: String,
    pub app_name: String,
    pub port: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            service_name: ACCOUNT.into(),
            app_name: Uuid::new_v4().to_string(),
            port: env::var(ACCOUNT_PORT).unwrap_or("5050".into()),
        }
    }
}
