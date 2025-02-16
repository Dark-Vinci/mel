use {
    async_trait::async_trait,
    sdk::{
        errors::GrpcError,
        models::{
            db::channel::{
                channel::Model as Channel, channel_user::Model as ChannelUser,
            },
            others::{
                auth::channel::{
                    CreateChannel, CreateChannelUser, UpdateChannel,
                },
                Paginated, Pagination,
            },
        },
    },
    uuid::Uuid,
};

#[async_trait]
pub trait ChannelTrait {
    async fn create_channel(
        &self,
        payload: CreateChannel,
        request_id: Uuid,
    ) -> Result<Channel, GrpcError>;

    async fn update_channel(
        &self,
        payload: UpdateChannel,
        request_id: Uuid,
    ) -> Result<Channel, GrpcError>;

    async fn delete_channel(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<(), GrpcError>;
}

pub trait Account {}

#[async_trait]
pub trait ChannelUserTrait {
    async fn remove_channel_user(
        &self,
        channel_user_id: Uuid,
        request_id: Uuid,
    ) -> Result<(), GrpcError>;

    async fn get_channel_user(
        &self,
        channel_id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> Result<Paginated<Vec<ChannelUser>>, GrpcError>;

    async fn create_channel_user(
        &self,
        payload: CreateChannelUser,
        request_id: Uuid,
    ) -> Result<ChannelUser, GrpcError>;
}

pub trait Settings {}

#[async_trait]
pub trait BookMarkTrait {}

#[async_trait]
pub trait PinTrait {}
