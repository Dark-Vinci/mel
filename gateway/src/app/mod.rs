use {
    crate::{config, downstream::Downstream},
    axum::async_trait,
    sdk::utils::redis::{MyRedis, MyRedisImpl},
};

#[derive(Clone, Debug)]
pub struct App {
    config: config::ApplicationConfig,
    downstream: Downstream,
    redis: Box<dyn MyRedisImpl>,
}

impl App {
    async fn new() -> Self {
        let r = MyRedis::new("url".into(), "".to_string()).await;

        Self {
            config: Default::default(),
            downstream: Downstream::new(),
            redis: Box::new(r),
        }
    }
}

impl Account for App {
    async fn get_user_by_id(&self, id: String) -> Self {
        let a = self.redis.get_value(id).await;

        "this is the string".into()
    }
}

#[async_trait]
pub trait Account {
    async fn get_user_by_id(&self, id: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() {
        let app = App::new();
        app.get_user_by_id("21".into())
    }
}
