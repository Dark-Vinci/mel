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
    pub fn new(app: AppConfig) -> Self {
        Self {
            kafka: KafkaConfig {
                url: "".to_string(),
                topic: "".to_string(),
                consumer_group: "".to_string(),
            },
            app,
            redis: RedisConfig {
                password: "".to_string(),
                username: "".to_string(),
                host: "".to_string(),
                port: "".to_string(),
            },
            db: DBConfig {
                password: "".to_string(),
                username: "".to_string(),
                host: "".to_string(),
            },
            environment: Environment::default(),
        }
    }
}
