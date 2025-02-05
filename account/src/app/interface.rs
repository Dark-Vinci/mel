use {
    async_trait::async_trait,
    sdk::{
        errors::GrpcError,
        models::{
            db::account::{
                channel::Model as Channel, channel_user::Model as ChannelUser,
            },
            others::auth::channel::{CreateChannel, CreateChannelUser},
        },
    },
    uuid::Uuid,
};

#[async_trait]
pub trait Auth {}

pub trait Account {}

pub trait Settings {}

#[async_trait]
pub trait ChannelTrait {
    async fn create_channel(
        &self,
        payload: CreateChannel,
        request_id: Uuid,
    ) -> Result<Channel, GrpcError>;

    async fn create_channel_user(
        &self,
        payload: CreateChannelUser,
        request_id: Uuid,
    ) -> Result<ChannelUser, GrpcError>;

    async fn delete_channel(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<(), GrpcError>;
}

pub trait AccountInterface: Auth + Account + Settings + ChannelTrait {}
