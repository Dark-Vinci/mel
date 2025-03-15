use {
    async_trait::async_trait,
    sdk::generated_proto_rs::{
        mel_extras::extras_service_client::ExtrasServiceClient,
        mel_messaging::messaging_service_client::MessagingServiceClient,
    },
    tonic::transport::Channel,
    tracing::error,
};

#[derive(Clone, Debug)]
pub struct Extras {
    config: String,
    connection: Option<ExtrasServiceClient<Channel>>,
}

impl Extras {
    pub fn new(config: String) -> Self {
        Self {
            config,
            connection: None,
        }
    }

    pub async fn get_connection(
        &mut self,
    ) -> Option<ExtrasServiceClient<Channel>> {
        if self.connection.is_none() {
            if let Err(err) = self.connect().await {
                error!(
                    "Error connecting to ExtrasServiceClient again: {:?}",
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

        let channel = Channel::from_static(&self.config).connect().await?; // todo; change the error to client connection error

        let client = ExtrasServiceClient::new(channel);

        self.connection = Some(client);

        Ok(())
    }
}

#[async_trait]
pub trait ExtrasOperations {}
