use {
    crate::{
        app::interfaces::{Account, AppInterface},
        config::config::Config,
        downstream::{Downstream, DownstreamInterface},
        models::context::Ctx,
    },
    async_trait::async_trait,
    sdk::utils::{
        objects::{ObjectStore, S3},
        redis::{MyRedis, RedisInterface},
    },
    std::sync::Arc,
};

#[derive(Clone)]
pub struct App {
    pub config: Config,
    pub downstream: Arc<dyn DownstreamInterface + Sync + Send>,
    pub redis: Arc<dyn RedisInterface + Send + Sync>,
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

#[async_trait]
impl Account for App {
    async fn login_user(&self, _ctx: Ctx, _payload: String) -> String {
        todo!()
    }

    async fn forget_password(&self, _ctx: Ctx, _payload: String) -> String {
        todo!()
    }

    async fn create_user(&self, _ctx: Ctx, _payload: String) -> String {
        todo!()
    }
}

impl AppInterface for App {}
