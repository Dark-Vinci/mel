use {
    crate::config::{
        app::AppConfig, db::DBConfig, kafka::KafkaConfig, redis::RedisConfig,
    },
    sdk::constants::Environment,
    std::env,
};

#[derive(Debug)]
pub struct Config {
    pub kafka: KafkaConfig,
    pub app: AppConfig,
    pub redis: RedisConfig,
    pub db: DBConfig,
    pub environment: Environment,
    pub auth_token: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            app: AppConfig::new(),
            kafka: KafkaConfig::new(),
            redis: RedisConfig::new(),
            db: DBConfig::new(),
            environment: Environment::default(),
            auth_token: env::var("AUTH_TOKEN")
                .expect("AUTH_TOKEN env variable is required"),
        }
    }
}
