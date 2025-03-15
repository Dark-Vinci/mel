use {
    crate::downstream::channel::channel::{Channel, ChannelOperations},
    async_trait::async_trait,
};

#[async_trait]
impl ChannelOperations for Channel {}
