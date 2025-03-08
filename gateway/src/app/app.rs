use {
    crate::{
        app::interfaces::{Account, AppInterface},
        config::config::Config,
        downstream::{Downstream, DownstreamInterface},
        models::context::CTX,
    },
    sdk::utils::{
        objects::{ObjectStore, S3},
        redis::{MyRedis, RedisInterface},
    },
    std::sync::Arc,
};

#[derive(Clone)]
pub struct App {
    config: Config,
    downstream: Arc<dyn DownstreamInterface + Sync + Send>,
    redis: Arc<dyn RedisInterface + Send + Sync>,
    pub object_store: Arc<dyn ObjectStore + Send + Sync>,
}

impl App {
    pub async fn new(c: Config) -> Self {
        let r = MyRedis::new("url", "", "", "", "").await;
        let object_store = S3::new("", "", "", "");

        Self {
            config: c,
            downstream: Arc::new(Downstream::new()),
            redis: Arc::new(r),
            object_store: Arc::new(object_store),
        }
    }
}

impl Account for App {
    async fn login_user(ctx: CTX, payload: String) -> String {
        todo!()
    }

    async fn forget_password(ctx: CTX, payload: String) -> String {
        todo!()
    }

    async fn create_user(ctx: CTX, payload: String) -> String {
        todo!()
    }
}

impl AppInterface for App {}
