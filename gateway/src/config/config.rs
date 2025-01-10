use crate::config::{app::App, downstream::Downstream, redis::Redis};

#[derive(Clone, Debug)]
pub struct Config {
    pub app: App,
    pub downstream: Downstream,
    pub redis: Redis,
}

impl Config {
    pub fn new() -> Self {
        let app = App::new();
        let downstream = Downstream::new();
        let redis = Redis::new();

        Self {
            app,
            downstream,
            redis,
        }
    }
}
