use {
    sdk::constants::constant::{
        KAFKA_GROUP_ID, KAFKA_HOST, KAFKA_PASSWORD, KAFKA_PORT, KAFKA_TOPIC,
        KAFKA_USERNAME,
    },
    std::env,
};

#[derive(Debug, Clone)]
pub struct KafkaConfig {
    pub group_id: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub topic: String,
}

impl KafkaConfig {
    pub fn new() -> Self {
        Self {
            group_id: env::var(KAFKA_GROUP_ID).unwrap_or_default(),
            username: env::var(KAFKA_USERNAME).unwrap_or_default(),
            password: env::var(KAFKA_PASSWORD).unwrap_or_default(),
            host: env::var(KAFKA_HOST).unwrap_or_default(),
            port: env::var(KAFKA_PORT).unwrap_or_default(),
            topic: env::var(KAFKA_TOPIC).unwrap_or_default(),
        }
    }
}
