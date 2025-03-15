use {
    async_trait::async_trait,
    sdk::generated_proto_rs::mel_account::account_service_client::AccountServiceClient,
    tonic::transport::{Channel, Uri},
    tracing::error,
};

#[derive(Clone, Debug)]
pub struct Account {
    config: String,
    connection: Option<AccountServiceClient<Channel>>,
}

impl Account {
    pub fn new(config: String) -> Self {
        Self {
            config,
            connection: None,
        }
    }

    pub async fn get_connection(
        &mut self,
    ) -> Option<AccountServiceClient<Channel>> {
        if self.connection.is_none() {
            if let Err(err) = self.connect().await {
                error!(
                    "Error connecting to ChannelServiceClient again: {:?}",
                    err
                );
                return None;
            }
        }

        self.connection.clone()
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.connection.is_some() {
            return Ok(());
        }

        let uri = Uri::try_from(&self.config)?;

        let channel = Channel::builder(uri).connect().await?; // todo; change the error to client connection error

        let client = AccountServiceClient::new(channel);

        self.connection = Some(client);

        Ok(())
    }
}

#[async_trait]
pub trait AccountOperations {}
