use {
    crate::connections::db::DB,
    async_trait::async_trait,
    chrono::Utc,
    sdk::{
        errors::RepoError,
        models::{
            db::messaging::message::{
                ActiveModel, Entity as MessageEntity, Model as Message,
            },
            others::{
                messaging::{CreateMessage, UpdateMessage},
                Paginated, Pagination,
            },
        },
    },
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait,
        IntoActiveModel, PaginatorTrait, QuerySelect,
    },
    tracing::{debug, error},
    uuid::Uuid,
};
use sdk::errors::RepoResult;

#[async_trait]
pub trait MessageRepository {
    async fn create(
        &self,
        payload: CreateMessage,
        request_id: Uuid,
    ) -> RepoResult<Message>;

    async fn update(
        &self,
        payload: UpdateMessage,
        request_id: Uuid,
    ) -> RepoResult<Message>;

    async fn delete(&self, id: Uuid, request_id: Uuid)
        -> RepoResult<()>;

    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<Message>;

    async fn get_for_channel(
        &self,
        channel_id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> RepoResult<Paginated<Vec<Message>>>;
}

pub struct MessageRepo(DB);

impl MessageRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl MessageRepository for MessageRepo {
    #[tracing::instrument(skip(self), name = "MessageRepo::create")]
    async fn create(
        &self,
        payload: CreateMessage,
        request_id: Uuid,
    ) -> RepoResult<Message> {
        debug!(
            "Got Create Message request with request id {} and payload {}",
            request_id, payload
        );

        let model: ActiveModel = payload.into();

        let result = model.insert(&self.0.connection).await.map_err(|err| {
            error!("Failed to create message with error {err}");

            if let Err(DbErr::RecordNotInserted) = err {
                return RepoError::FailedToInsert;
            }

            return RepoError::SomethingWentWrong;
        })?;

        Ok(result)
    }

    #[tracing::instrument(skip(self), name = "MessageRepo::update")]
    async fn update(
        &self,
        payload: UpdateMessage,
        request_id: Uuid,
    ) -> RepoResult<Message> {
        debug!(
            "Got Update Message request with request id {} and payload {}",
            request_id, payload
        );

        let model: ActiveModel = payload.into();

        let result = model.update(&self.0.connection).await.map_err(|err| {
            error!("Failed to update message with error {err}");

            if let Err(DbErr::RecordNotUpdated) = err {
                error!("Failed to update message with error {err}");

                return RepoError::FailedToUpdate;
            }

            RepoError::SomethingWentWrong
        })?;

        Ok(result)
    }

    #[tracing::instrument(skip(self), name = "MessageRepo::delete")]
    async fn delete(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<(), RepoError> {
        debug!(
            "Got Delete Message request with request id {} and id {}",
            request_id, id
        );

        let mut message =
            self.get_by_id(id, request_id).await?.into_active_model();

        message.deleted_at = Set(Some(Utc::now()));

        let _ = message.update(&self.0.connection).await.map_err(|err| {
            error!("Failed to update message with error {err}");

            RepoError::SomethingWentWrong
        })?;

        Ok(())
    }

    #[tracing::instrument(skip(self), name = "MessageRepo::get_by_id")]
    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<Message> {
        debug!(
            "Got Get Message request with request id {} and id {}",
            request_id, id
        );

        let message = MessageEntity::find_by_id(id)
            .one(&self.0.connection)
            .await
            .map_err(|err| {
                error!("Failed to get user by id with error {err}");

                return RepoError::NotFound;
            })?;

        if message.is_none() {
            error!("Failed to get user by id");
            return Err(RepoError::NotFound);
        }

        Ok(message.unwrap())
    }

    #[tracing::instrument(skip(self), name = "MessageRepo::get_for_channel")]
    async fn get_for_channel(
        &self,
        channel_id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> Result<Paginated<Vec<Message>>, RepoError> {
        debug!(
            "Got Get For Channel request with request id {} and id {}",
            request_id, channel_id
        );

        let result = MessageEntity::find()
            .limit(Some(pagination.page_size))
            .offset(pagination.page_offset())
            .all(&self.0.connection)
            .await
            .map_err(|err| {
                error!("Unable to query message by range with error {err}");

                RepoError::SomethingWentWrong
            })?;

        let count = MessageEntity::find()
            .count(&self.0.connection)
            .await
            .map_err(|err| {
                error!("Unable to count message by range with error {err}");

                RepoError::SomethingWentWrong
            })?;

        let paginated =
            Paginated::new(result, pagination.total_pages(count), 0, 0, count);

        Ok(paginated)
    }
}
