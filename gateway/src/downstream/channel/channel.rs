
use async_trait::async_trait;
use tonic::transport::Channel as TonicChannel;
use tracing::error;
use sdk::generated_proto_rs::mel_channel::channel_service_client::ChannelServiceClient;
use sdk::generated_proto_rs::mel_extras::extras_service_client::ExtrasServiceClient;

#[derive(Clone, Debug)]
pub struct Channel {
    config: String,
    connection: Option<ChannelServiceClient<TonicChannel>>,
}

impl Channel {
    pub fn new(config: String) -> Self {
        Self{
            config,
            connection: None,
        }
    }

    pub async fn get_connection(&mut self) -> Option<ChannelServiceClient<TonicChannel>> {
        if self.connection.is_none() {
            if let Err(err) = self.connect().await {
                error!("Error connecting to ChannelServiceClient again: {:?}", err);
                return None;
            }
        }

        self.connection.clone()
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.connection.is_some() {
            return Ok(());
        }

        let channel = TonicChannel::from_static(&self.config)
            .connect()
            .await?; // todo; change the error to client connection error

        let client = ChannelServiceClient::new(channel);

        self.connection = Some(client);

        Ok(())
    }
}

#[async_trait]
pub trait ChannelOperations{}
