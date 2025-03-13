use {
    crate::connections::db::DB,
    async_trait::async_trait,
    chrono::Utc,
    sdk::{
        errors::{RepoError, RepoResult},
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
        current: ActiveModel,
        request_id: Uuid,
    ) -> RepoResult<Message>;

    async fn delete(
        &self,
        current: ActiveModel,
        request_id: Uuid,
    ) -> RepoResult<()>;

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
            request_id = %request_id,
            "Got Create Message request to create message",
        );

        let model: ActiveModel = payload.into();

        let result = model.insert(&self.0.connection).await.map_err(|err| {
            error!(
                debug_error = ?err,
                display_error = %err,
                "failure, unable to create message"
            );

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
        current: ActiveModel,
        request_id: Uuid,
    ) -> RepoResult<Message> {
        debug!(
            request_id = %request_id,
            "Got request to update message"
        );

        let model: ActiveModel = (payload, current).into();

        let result = model.update(&self.0.connection).await.map_err(|err| {
            error!(
                debug_error = ?err,
                display_error = %err,
                "failure, unable  to update message"
            );

            if let Err(DbErr::RecordNotUpdated) = err {
                return RepoError::FailedToUpdate;
            }

            RepoError::SomethingWentWrong
        })?;

        Ok(result)
    }

    #[tracing::instrument(skip(self), name = "MessageRepo::delete")]
    async fn delete(
        &self,
        mut message: ActiveModel,
        request_id: Uuid,
    ) -> Result<(), RepoError> {
        debug!(
            request_id = %request_id,
            "Got request to delete message"
        );

        message.deleted_at = Set(Some(Utc::now()));

        let _ = message.update(&self.0.connection).await.map_err(|err| {
            error!(
                debug_error = ?err,
                display_error = %err,
                "failure, unable  to delete message"
            );

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
            request_id = %request_id,
            "Got a request to delete message"
        );

        let message = MessageEntity::find_by_id(id)
            .one(&self.0.connection)
            .await
            .map_err(|err| {
                error!(
                    debug_error = ?err,
                    display_error = %err,
                    "failure, unable  to get message by id"
                );

                return RepoError::NotFound;
            })?;

        if message.is_none() {
            error!("Fail, unable to get message by id");
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
            request_id = %request_id,
            "Got request to get channel messages"
        );

        // todo; make this requests in parallel
        let result = MessageEntity::find()
            .limit(Some(pagination.page_size))
            .offset(pagination.page_offset())
            .all(&self.0.connection)
            .await
            .map_err(|err| {
                error!(
                    debug_error = ?err,
                    display_error = %err,
                    "failure, Unable to query message by range with error"
                );

                RepoError::SomethingWentWrong
            })?;

        let count = MessageEntity::find()
            .count(&self.0.connection)
            .await
            .map_err(|err| {
                error!(
                    debug_error = ?err,
                    display_error = %err,
                    "failure, Unable to count message by range"
                );

                RepoError::SomethingWentWrong
            })?;

        let paginated = Paginated::new(
            result,
            pagination.total_pages(count),
            pagination.page_number + 1,
            pagination.page_size,
            count,
        );

        Ok(paginated)
    }
}
