use {
    crate::connections::db::DB,
    async_trait::async_trait,
    chrono::Utc,
    sdk::{
        errors::{RepoError, RepoResult},
        models::{
            db::channel::channel::{
                ActiveModel, Column, Entity as ChannelEntity, Model as Channel,
            },
            others::auth::channel::{CreateChannel, UpdateChannel},
        },
    },
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait,
        IntoActiveModel, QueryFilter,
    },
    tracing::{debug, error},
    uuid::Uuid,
};

#[async_trait]
pub trait ChannelRepository {
    async fn create(
        &self,
        payload: CreateChannel,
        request_id: Uuid,
    ) -> RepoResult<Channel>;

    async fn update(
        &self,
        payload: UpdateChannel,
        request_id: Uuid,
    ) -> RepoResult<Channel>;

    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<Channel>;

    async fn delete(&self, id: Uuid, request_id: Uuid)
        -> Result<(), RepoError>;

    async fn get_by_name(
        &self,
        name: &str,
        request_id: Uuid,
    ) -> RepoResult<Channel>;
}

pub struct ChannelRepo(DB);

impl ChannelRepo {
    pub fn new(connection: DB) -> Self {
        Self(connection)
    }
}

#[async_trait]
impl ChannelRepository for ChannelRepo {
    #[tracing::instrument(name = "ChannelRepo::create", skip(self))]
    async fn create(
        &self,
        payload: CreateChannel,
        request_id: Uuid,
    ) -> RepoResult<Channel> {
        debug!(
            "Creating channel: {:?}, with request id: {}",
            payload, request_id
        );

        let a: ActiveModel = payload.into();

        let result = a.insert(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = result {
            error!(error = &err.to_string(), "Failed to insert channel record");

            if err.to_string().contains("duplicate key") {
                return Err(RepoError::DuplicateKey);
            }

            return Err(RepoError::FailedToInsert);
        }

        Ok(result.unwrap())
    }

    #[tracing::instrument(name = "ChannelRepo::update", skip(self))]
    async fn update(
        &self,
        payload: UpdateChannel,
        request_id: Uuid,
    ) -> RepoResult<Channel> {
        debug!("Updating chan by id: {:?}, request_id {}", payload, request_id);

        let model: ActiveModel = payload.into();

        let result = model.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to update chan");
            return Err(RepoError::FailedToUpdate);
        }

        Ok(result.unwrap())
    }

    #[tracing::instrument(name = "ChannelRepo::get_by_id", skip(self))]
    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<Channel> {
        debug!(
            "Getting channel by id: {}, with request id: {}",
            id, request_id
        );

        let result =
            ChannelEntity::find_by_id(id).one(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to find channel by id");
            return Err(RepoError::SomethingWentWrong);
        }

        let chan = result.unwrap();

        if chan.is_none() {
            error!("channel not found");
            return Err(RepoError::NotFound);
        }

        Ok(chan.unwrap())
    }

    #[tracing::instrument(name = "ChannelRepo::delete", skip(self))]
    async fn delete(&self, id: Uuid, request_id: Uuid) -> RepoResult<()> {
        debug!("Deleting channel by id: {}, request_id {}", id, request_id);

        let mut result =
            self.get_by_id(request_id, id).await?.into_active_model();

        result.deleted_at = Set(Some(Utc::now()));

        let res = result.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &res {
            error!(error = &err.to_string(), "Failed to find channel by mail");
            return Err(RepoError::SomethingWentWrong);
        }

        Ok(())
    }

    #[tracing::instrument(name = "ChannelRepo::get_by_name", skip(self))]
    async fn get_by_name(
        &self,
        name: &str,
        request_id: Uuid,
    ) -> RepoResult<Channel> {
        debug!(
            "Getting channel by id: {}, with request id: {}",
            name, request_id
        );

        let result = ChannelEntity::find()
            .filter(Column::Name.eq(name))
            .one(&self.0.connection)
            .await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to find channel by id");
            return Err(RepoError::SomethingWentWrong);
        }

        let chan = result.unwrap();

        if chan.is_none() {
            error!("channel not found");
            return Err(RepoError::NotFound);
        }

        Ok(chan.unwrap())
    }
}
