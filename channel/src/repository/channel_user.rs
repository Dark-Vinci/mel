use {
    crate::{connections::db::DB, repository::channel_user},
    async_trait::async_trait,
    chrono::Utc,
    sdk::{
        errors::RepoError,
        models::{
            db::channel::channel_user::{
                ActiveModel, Column, Entity as ChannelUserEntity,
                Model as ChannelUser,
            },
            others::{
                auth::channel::{CreateChannelUser, UpdateChannelUser},
                Paginated, Pagination,
            },
        },
    },
    sea_orm::{
        entity::*, query::*, ActiveModelTrait, ActiveValue::Set, ColumnTrait,
        Condition, DbErr, EntityTrait, IntoActiveModel,
    },
    tracing::{debug, error},
    uuid::Uuid,
};
use sdk::errors::RepoResult;

#[async_trait]
pub trait ChannelUserRepository {
    async fn create(
        &self,
        payload: CreateChannelUser,
        request_id: Uuid,
    ) -> RepoResult<ChannelUser>;

    async fn update(
        &self,
        payload: UpdateChannelUser,
        request_id: Uuid,
    ) -> RepoResult<ChannelUser>;

    async fn delete(&self, id: Uuid, request_id: Uuid)
        -> Result<(), RepoError>;

    async fn get(
        &self,
        id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> Result<Paginated<Vec<ChannelUser>>, RepoError>;

    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<ChannelUser>;
}

pub struct ChannelUserRepo(DB);

impl ChannelUserRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl ChannelUserRepository for ChannelUserRepo {
    #[tracing::instrument(name = "ChannelUserRepo::create", skip(self))]
    async fn create(
        &self,
        payload: CreateChannelUser,
        request_id: Uuid,
    ) -> RepoResult<ChannelUser> {
        debug!(
            "Creating channel_user by id: {:?}, request_id {}",
            payload, request_id
        );

        let model: ActiveModel = payload.into();

        let result = model.insert(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to create channel_user");
            return Err(RepoError::FailedToUpdate);
        }

        Ok(result.unwrap())
    }

    #[tracing::instrument(name = "ChannelUserRepo::update", skip(self))]
    async fn update(
        &self,
        payload: UpdateChannelUser,
        request_id: Uuid,
    ) -> RepoResult<ChannelUser> {
        debug!(
            "Updating channel_user by id: {:?}, request_id {}",
            payload, request_id
        );

        let model: ActiveModel = payload.into();

        let result = model.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to update channel_user");
            return Err(RepoError::FailedToUpdate);
        }

        Ok(result.unwrap())
    }

    #[tracing::instrument(name = "ChannelUserRepo::delete", skip(self))]
    async fn delete(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<()> {
        debug!(
            "Deleting channel_user by id: {}, request_id {}",
            id, request_id
        );

        let mut channel_user =
            self.get_by_id(id, request_id).await?.into_active_model();

        channel_user.deleted_at = Set(Some(Utc::now()));

        let res = channel_user.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &res {
            error!(
                error = &err.to_string(),
                "Failed to find channel_user by mail"
            );
            return Err(RepoError::SomethingWentWrong);
        }

        Ok(())
    }

    #[tracing::instrument(name = "ChannelUserRepo::get", skip(self))]
    async fn get(
        &self,
        id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> RepoResult<Paginated<Vec<ChannelUser>>> {
        debug!(
            "getting channel users by id: {}, request_id {}",
            id, request_id
        );

        let result = ChannelUserEntity::find()
            .limit(Some(pagination.page_size)) // Set limit
            .offset((pagination.page_number - 1) * pagination.page_size) // Set offset
            .all(&self.0.connection) // Execute query
            .await;

        if let Err(DbErr::Query(err)) = &result {
            error!(
                error = &err.to_string(),
                "Failed to find channel_user by id"
            );
            return Err(RepoError::SomethingWentWrong);
        }

        let count = ChannelUserEntity::find().count(&self.0.connection).await;

        if let Err(DbErr::Query(err)) = &count {
            error!(
                error = &err.to_string(),
                "Failed to find channel_user by id"
            );
            return Err(RepoError::SomethingWentWrong);
        }

        let count = count.unwrap();

        let total_pages =
            (count + pagination.page_size - 1) / pagination.page_size;

        let paginated = Paginated {
            result: result.unwrap(),
            total_pages,
            current_page: 0,
            page_size: 0,
            total_items: count,
        };

        Ok(paginated)
    }

    #[tracing::instrument(name = "ChannelUserRepo::get_by_id", skip(self))]
    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<ChannelUser> {
        debug!(
            "Getting channel_user by id: {}, with request id: {}",
            id, request_id
        );

        let result = ChannelUserEntity::find_by_id(id)
            .one(&self.0.connection)
            .await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(
                error = &err.to_string(),
                "Failed to find channel_user by id"
            );
            return Err(RepoError::SomethingWentWrong);
        }

        let result = result.unwrap();

        if result.is_none() {
            error!("channel_user not found");
            return Err(RepoError::NotFound);
        }

        Ok(result.unwrap())
    }
}
