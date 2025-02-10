use {
    crate::app::{
        app::App,
        interface::{ChannelTrait, ChannelUserTrait},
    },
    async_trait::async_trait,
    sdk::{
        errors::GrpcError,
        models::{
            db::channel::{
                channel::Model as Channel, channel_user::Model as ChannelUser,
            },
            others::{
                auth::channel::{
                    CreateChannel, CreateChannelUser, UpdateChannel,
                },
                Paginated, Pagination,
            },
        },
    },
    tracing::{debug, error},
    uuid::Uuid,
};

#[async_trait]
impl ChannelTrait for App {
    #[tracing::instrument(name = "App::create_channel", skip(self))]
    async fn create_channel(
        &self,
        payload: CreateChannel,
        request_id: Uuid,
    ) -> Result<Channel, GrpcError> {
        debug!("App::create_channel; Got Request to create new channel: {payload}, request_id: {request_id}");

        let channel = self
            .channel_repo
            .get_by_name("request_id", request_id)
            .await;

        if channel.is_ok() {
            error!(?request_id, ?channel, "Channel already exists");
            return Err(GrpcError::AlreadyExists("Channel".into())); // channel already exists
        }

        let channel = self.channel_repo.create(payload, request_id).await;

        // make this user an admin
        let channel_ = CreateChannelUser {};

        let _channel_user =
            self.create_channel_user(channel_, request_id).await;

        if let Err(_err) = channel {
            error!(?request_id, ?channel, "Unable to create channel");
            return Err(GrpcError::Generic); // unable to create;
        }

        Ok(channel.unwrap())
    }

    #[tracing::instrument(name = "App::update_channel", skip(self))]
    async fn update_channel(
        &self,
        payload: UpdateChannel,
        request_id: Uuid,
    ) -> Result<Channel, GrpcError> {
        debug!("App::update_channel; Got Request to update channel");

        let _ = self
            .channel_repo
            .get_by_id(payload.id, request_id)
            .await
            .map_err(|err| {
                error!(?request_id, ?err, "Unable to retrieve channel");

                GrpcError::NotFound("Channel".into())
            })?;

        let updated_channel = self
            .channel_repo
            .update(payload, request_id)
            .await
            .map_err(|err| {
                error!(?request_id, ?err, "Unable to update channel");

                GrpcError::Generic
            })?;

        Ok(updated_channel)
    }

    #[tracing::instrument(name = "App::delete_channel", skip(self))]
    async fn delete_channel(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<(), GrpcError> {
        debug!("App::delete_channel; Got Request to delete channel");

        let channel =
            self.channel_repo.get_by_id(id, request_id).await.map_err(
                |_err| {
                    error!("Unable to find channel by id: {id}, err{_err}");
                    GrpcError::Generic
                },
            )?;

        // delete all channel_users

        // delete all channel {messages, documents, ....}

        let _ = self
            .channel_repo
            .delete(channel.id, request_id)
            .await
            .map_err(|_err| {
                error!("Unable to find channel by id: {id}, err{_err}");
                GrpcError::Generic
            })?;

        Ok(())
    }
}
