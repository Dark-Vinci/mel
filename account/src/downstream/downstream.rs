use uuid::Uuid;

#[async_trait::async_trait]
pub trait Downstream {
    async fn ping(request_id: Uuid);
}

pub struct DownstreamImpl {}

impl DownstreamImpl {
    pub fn new() -> Self {
        todo!()
    }
}

#[async_trait::async_trait]
impl Downstream for DownstreamImpl {
    async fn ping(request_id: Uuid) {
        todo!()
    }
}
