use {
    crate::{
        app::interfaces::{Account, AppInterface},
        config::config::Config,
        downstream::{Downstream, DownstreamInterface},
    },
    sdk::utils::redis::{MyRedis, RedisInterface},
    std::sync::Arc,
};

#[derive(Clone)]
pub struct App {
    config: Config,
    downstream: Arc<dyn DownstreamInterface + Sync + Send>,
    redis: Arc<dyn RedisInterface + Send + Sync>,
}

impl App {
    pub async fn new(c: Config) -> Self {
        let r = MyRedis::new("url", "", "", "", "").await;

        Self {
            config: c,
            downstream: Arc::new(Downstream::new()),
            redis: Arc::new(r),
        }
    }
}

impl Account for App {}

impl AppInterface for App {}
