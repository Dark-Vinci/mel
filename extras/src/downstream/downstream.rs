use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Downstream {
    async fn ping(&self, request_id: Uuid);
}

pub struct DownstreamImpl {}

impl DownstreamImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Downstream for DownstreamImpl {
    async fn ping(&self, _request_id: Uuid) {
        todo!()
    }
}
