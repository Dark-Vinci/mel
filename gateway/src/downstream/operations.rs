use crate::downstream::{
    account::account::AccountOperations,
    channel::channel::ChannelOperations,
    downstream::{Downstream, DownstreamOperations},
    extras::extras::ExtrasOperations,
    messaging::messaging::MessagingOperations,
};

impl MessagingOperations for Downstream {}

impl ExtrasOperations for Downstream {}

impl ChannelOperations for Downstream {}

impl AccountOperations for Downstream {}

impl DownstreamOperations for Downstream {}
