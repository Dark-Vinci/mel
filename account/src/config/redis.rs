#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub password: String,
    pub username: String,
    pub host: String,
    pub port: String,
}
