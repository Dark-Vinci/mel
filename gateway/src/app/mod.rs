use {
    crate::{config, downstream::Downstream},
    axum::async_trait,
    sdk::utils::redis::{MyRedis, RedisInterface},
};

#[derive(Clone, Debug)]
pub struct App {
    config: config::ApplicationConfig,
    downstream: Downstream,
    redis: Box<dyn RedisInterface>,
}

impl App {
    pub async fn new() -> Self {
        let r = MyRedis::new("url".into(), "".to_string()).await;

        Self {
            config: Default::default(),
            downstream: Downstream::new(),
            redis: Box::new(r),
        }
    }
}

impl Account for App {}

pub trait AppInterface: Account {}

#[async_trait]
pub trait Account {}
