use crate::downstream::account::account::AccountOperations;
use crate::downstream::channel::channel::ChannelOperations;
use crate::downstream::downstream::{Downstream, DownstreamOperations};
use crate::downstream::extras::extras::ExtrasOperations;
use crate::downstream::messaging::messaging::MessagingOperations;

impl MessagingOperations for Downstream {}

impl ExtrasOperations for Downstream {}

impl ChannelOperations for Downstream {}

impl AccountOperations for Downstream {}

impl DownstreamOperations for Downstream {}