use {async_trait::async_trait, sdk::errors::GrpcError};

pub trait Auth {}

#[async_trait]
pub trait Account {
    async fn name(&self) -> Result<String, GrpcError>;
}

pub trait Settings {}
