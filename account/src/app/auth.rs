use {
    crate::app::{app::App, interface::Auth},
    async_trait::async_trait,
    sdk::{
        errors::GrpcError,
        models::{
            db::account::channel::Model as Channel,
            others::auth::channel::CreateChannel,
        },
    },
    tracing::{debug, error},
    uuid::Uuid,
};

#[async_trait]
impl Auth for App {
    #[tracing::instrument(name = "App::create_channel", skip(self))]
    async fn create_channel(
        &self,
        payload: CreateChannel,
        request_id: Uuid,
    ) -> Result<Channel, GrpcError> {
        debug!("App::create_channel; Got Request to create new channel: {payload}, request_id: {request_id}");

        let channel = self.channel_repo.get_by_id(request_id, request_id).await; //  get by channel {name, channel_id}

        if channel.is_ok() {
            error!(?request_id, ?channel, "Account already exists");
            return Err(GrpcError::Generic); // channel already exists
        }

        let channel = self.channel_repo.create(payload, request_id).await;

        // add the user to the channel {channel_user}

        if let Err(err) = channel {
            error!(?request_id, ?channel, "Unable to create channel");
            return Err(GrpcError::Generic); // unable to create;
        }

        Ok(channel.unwrap())
    }
}
