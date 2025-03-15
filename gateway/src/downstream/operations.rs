use {
    crate::downstream::{
        account::account::AccountOperations,
        channel::channel::ChannelOperations,
        downstream::{Downstream, DownstreamOperations},
        extras::extras::ExtrasOperations,
        messaging::messaging::MessagingOperations,
    },
    async_trait::async_trait,
};

#[async_trait]
impl MessagingOperations for Downstream {}

#[async_trait]
impl ExtrasOperations for Downstream {}

#[async_trait]
impl ChannelOperations for Downstream {}

#[async_trait]
impl AccountOperations for Downstream {}

#[async_trait]
impl DownstreamOperations for Downstream {}
