use {
    crate::config::{
        app::AppConfig, db::DBConfig, kafka::KafkaConfig, redis::RedisConfig,
    },
    sdk::constants::Environment,
};

#[derive(Debug)]
pub struct Config {
    pub kafka: KafkaConfig,
    pub app: AppConfig,
    pub redis: RedisConfig,
    pub db: DBConfig,
    pub environment: Environment,
}

impl Config {
    pub fn new() -> Self {
        Self {
            app: AppConfig::new(),
            kafka: KafkaConfig::new(),
            redis: RedisConfig::new(),
            db: DBConfig::new(),
            environment: Environment::default(),
        }
    }
}
