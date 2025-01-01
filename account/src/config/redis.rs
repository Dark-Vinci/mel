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
            password: "".into(),
            username: "".into(),
            host: "".into(),
            port: "".into(),
        }
    }
}
