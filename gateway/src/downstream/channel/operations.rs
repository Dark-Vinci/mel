use async_trait::async_trait;
use crate::downstream::channel::channel::{Channel, ChannelOperations};

#[async_trait]
impl ChannelOperations for Channel {}