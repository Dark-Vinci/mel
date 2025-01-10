use {
    crate::{
        app::interfaces::{Account, AppInterface},
        config::config::Config,
        downstream::{Downstream, DownstreamOperations},
    },
    sdk::utils::redis::{MyRedis, RedisInterface},
};

pub struct App {
    config: Config,
    downstream: Box<dyn DownstreamOperations>,
    redis: Box<dyn RedisInterface>,
}

impl App {
    pub async fn new(c: Config) -> Self {
        let r = MyRedis::new("url".into(), "".to_string()).await;

        Self {
            config: c,
            downstream: Box::new(Downstream::new()),
            redis: Box::new(r),
        }
    }
}

impl Account for App {}

impl AppInterface for App {}
