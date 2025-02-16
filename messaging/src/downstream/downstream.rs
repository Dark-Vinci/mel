use uuid::Uuid;

#[async_trait::async_trait]
pub trait Downstream {
    async fn ping(request_id: Uuid);
}
