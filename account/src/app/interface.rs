use {
    async_trait::async_trait,
    sdk::{
        errors::GrpcError,
        models::{
            db::account::channel::Model as Channel,
            others::auth::channel::CreateChannel,
        },
    },
    uuid::Uuid,
};

#[async_trait]
pub trait Auth {
    async fn create_channel(
        &self,
        payload: CreateChannel,
        request_id: Uuid,
    ) -> Result<Channel, GrpcError>;
}

pub trait Account {}

pub trait Settings {}

pub trait AccountInterface: Auth + Account + Settings {}
