pub trait Auth {}
use sdk::errors::GrpcError;

#[async_trait::async_trait]
pub trait Account {
    async fn name(&self) -> Result<String, GrpcError>;
}

pub trait Settings {}
