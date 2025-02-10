use {
    crate::app::{app::App, interface::ChannelUserTrait},
    async_trait::async_trait,
    sdk::{
        errors::GrpcError,
        models::{
            db::channel::channel_user::Model as ChannelUser,
            others::{auth::channel::CreateChannelUser, Paginated, Pagination},
        },
    },
    tracing::error,
    uuid::Uuid,
};

#[async_trait]
impl ChannelUserTrait for App {
    #[tracing::instrument(name = "App::remove_channel_user", skip(self))]
    async fn remove_channel_user(
        &self,
        channel_user_id: Uuid,
        request_id: Uuid,
    ) -> Result<(), GrpcError> {
        let _ = self
            .channel_user_repo
            .get_by_id(channel_user_id, request_id)
            .await
            .map_err(|err| {
                error!("Unable to find user by id: {channel_id}, err: {err}");
                GrpcError::NotFound(format!("Channel {channel_id} not found"))
            })?;

        let _ = self
            .channel_user_repo
            .delete(channel_user_id, request_id)
            .await
            .map_err(|err| {
                error!("Unable to find user by id: {channel_id}, err: {err}");
                GrpcError::Generic
            })?;

        Ok(())
    }

    #[tracing::instrument(name = "App::get_channel_user", skip(self))]
    async fn get_channel_user(
        &self,
        channel_id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> Result<Paginated<Vec<ChannelUser>>, GrpcError> {
        let _ = self
            .channel_repo
            .get_by_id(channel_id, request_id)
            .await
            .map_err(|err| {
                error!(?request_id, ?err, "Unable to retrieve channel");

                GrpcError::NotFound("Channel".into())
            })?;

        let channel_users = self
            .channel_user_repo
            .get(channel_id, pagination, request_id)
            .await
            .map_err(|err| {
                error!(?request_id, ?err, "Unable to retrieve channel users");

                GrpcError::NotFound("Channel".into())
            })?;

        Ok(channel_users)
    }

    #[tracing::instrument(name = "App::create_channel_user", skip(self))]
    async fn create_channel_user(
        &self,
        payload: CreateChannelUser,
        request_id: Uuid,
    ) -> Result<ChannelUser, GrpcError> {
        // check if the user is in the workspace
        // let _ = self
        //     .workspace_user_repo
        //     .get_by_id(request_id, request_id)
        //     .await
        //     .map_err(|_err| {
        //         error!("Error");
        //         GrpcError::Generic // update here
        //     })?; //

        let channel_user = self
            .channel_user_repo
            .create(payload, request_id)
            .await
            .map_err(|_err| {
                error!("Error");
                GrpcError::Generic
            })?;

        Ok(channel_user)
    }
}
