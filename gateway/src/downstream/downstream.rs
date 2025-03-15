use tracing::error;
use crate::config::config::Config;
use crate::downstream::account::account::{Account, AccountOperations};
use crate::downstream::channel::channel::{Channel, ChannelOperations};
use crate::downstream::extras::extras::{Extras, ExtrasOperations};
use crate::downstream::messaging::messaging::{Messaging, MessagingOperations};

#[derive(Clone, Debug)]
pub struct Downstream{
    config: Config,
    account: Account,
    messaging: Messaging,
    extras: Extras,
    channel: Channel,
}

impl Downstream {
    #[tracing::instrument]
    pub async fn connect(config: Config) -> Self {
        let mut account = Account::new(config.downstream.account_grpc_address.clone());
        if let Err(err) = account.connect().await {
            error!("Failed to connect to account service: {}", err);
        }

        let mut messaging = Messaging::new(config.downstream.account_grpc_address.clone());
        if let Err(err) = messaging.connect().await {
            error!("Failed to connect to messaging service: {}", err);
        }

        let mut extras = Extras::new(config.downstream.account_grpc_address.clone());
        if let Err(err) = extras.connect().await {
            error!("Failed to connect to extras service: {}", err);
        }

        let mut channel = Channel::new(config.downstream.account_grpc_address.clone());
        if let Err(err) = channel.connect().await {
            error!("Failed to connect to channel service: {}", err);
        }

        Self {
            config,
            account,
            messaging,
            extras,
            channel,
        }
    }
}

pub trait DownstreamOperations: MessagingOperations + ExtrasOperations + ChannelOperations + AccountOperations {}
