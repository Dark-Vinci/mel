use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, Condition, DbErr, EntityTrait, IntoActiveModel, PaginatorTrait, QuerySelect};
use tracing::{debug, error};
use sdk::{
    errors::{RepoError, RepoResult},
    models::{
        db::messaging::message::{Model as Message, Entity as MessageEntity},
        db::messaging::platform_user_message::{
            ActiveModel, Entity as PlatformUserMessageEntity,
            Model as PlatformUserMessage,
            Column,
        },
        others::{
            messaging::{CreateChat, UpdateChat},
            Paginated, Pagination,
        },
    },
};
use {
    crate::connections::db::DB,
    async_trait::async_trait,
    sdk::models::others::messaging::{
        CreatePlatformUserMessage, UpdatePlatformUserMessage,
    },
    sea_orm::tests_cfg::cake,
    uuid::Uuid,
};

struct UserMessage {
    platform_user_message: PlatformUserMessage,
    message: Option<Message>,
}

impl From<(PlatformUserMessage, Option<Message>)> for UserMessage {
    fn from(payload: (PlatformUserMessage, Option<Message>)) -> Self {
        Self {
            platform_user_message: payload.0,
            message: payload.1,
        }
    }
}

struct UserMessages {
    value: Vec<UserMessage>,
}

impl From<Vec<(PlatformUserMessage, Option<Message>)>> for UserMessages {
    fn from(payload: Vec<(PlatformUserMessage, Option<Message>)>) -> Self {
        let mut result = Self { value: Vec::new() };

        for v in payload {
            result.value.push(v.into())
        }

        result
    }
}

#[async_trait]
pub trait PlatformUserMessageRepository {
    async fn create(
        &self,
        payload: CreatePlatformUserMessage,
        request_id: Uuid,
    ) -> RepoResult<PlatformUserMessage>;

    async fn update(
        &self,
        payload: UpdatePlatformUserMessage,
        current: ActiveModel,
        request_id: Uuid,
    ) -> RepoResult<PlatformUserMessage>;

    async fn delete(
        &self,
        message: ActiveModel,
        request_id: Uuid,
    ) -> RepoResult<()>;

    async fn get_platform_message(
        &self,
        payload: QueryUserMessage,
        pagination: Pagination,
        request_id: Uuid,
    ) -> RepoResult<Paginated<UserMessages>>;
}

pub struct PlatformUserMessageRepositoryRepo(DB);

impl PlatformUserMessageRepositoryRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl PlatformUserMessageRepository for PlatformUserMessageRepositoryRepo {
    #[tracing::instrument(
        skip(self),
        name = "PlatformUserMessageRepository::create"
    )]
    async fn create(
        &self,
        payload: CreatePlatformUserMessage,
        request_id: Uuid,
    ) -> RepoResult<PlatformUserMessage> {
        debug!(
            request_id = %request_id,
            "Got request to create user message"
        );

        let user_message: ActiveModel = payload.into();

        let result = user_message.insert(&self.0.connection).await.map_err(|err| {
            error!(
                debug_error = ?err,
                display_error = %err,
                "Failed to insert into database"
            );

            RepoError::FailedToInsert
        })?;

        Ok(result)
    }

    #[tracing::instrument(skip(self), name = "PlatformUserMessageRepository::update")]
    async fn update(
        &self,
        payload: UpdatePlatformUserMessage,
        mut current: ActiveModel,
        request_id: Uuid,
    ) -> RepoResult<PlatformUserMessage> {
        debug!(
            request_id = %request_id,
            "Got request to update user message message"
        );

        let model: ActiveModel = (payload, current).into();

        let result = model.update(&self.0.connection).await.map_err(|err| {
            error!(
                debug_error = ?err,
                display_error = %err,
                "failure, unable  to update user message"
            );

            if let Err(DbErr::RecordNotUpdated) = err {
                return RepoError::FailedToUpdate;
            }

            RepoError::SomethingWentWrong
        })?;

        Ok(result)
    }

    #[tracing::instrument(skip(self), name = "PlatformUserMessageRepository::delete")]
    async fn delete(
        &self,
        mut message: ActiveModel,
        request_id: Uuid,
    ) -> RepoResult<()> {
        debug!(
            request_id = %request_id,
            "Got request to delete message"
        );

        message.deleted_at = Set(Some(Utc::now()));

        let _ = message.update(&self.0.connection).await.map_err(|err| {
            error!(
                debug_error = ?err,
                display_error = %err,
                "failure, unable  to delete user message"
            );

            RepoError::SomethingWentWrong
        })?;

        Ok(())
    }


    #[tracing::instrument(skip(self), name = "PlatformUserMessageRepository::get_platform_message")]
    async fn get_platform_message(
        &self,
        payload: QueryUserMessage,
        pagination: Pagination,
        request_id: Uuid,
    ) -> RepoResult<Paginated<UserMessages>> {
        debug!(
            request_id = %request_id,
            "Got request to get platform messages"
        );


        // suppose to be Mode, Option<Model>
        let result = PlatformUserMessageEntity::find()
            .find_also_related(MessageEntity)
            .filter(
                Condition::all()
                    .add(Column::PlatformId.eq(payload.platform_id))
                    .add(Column::UserId.eq(payload.user_id))
                    .add(Column::IsPrivateMessage.eq(payload.is_dm))
            )
            .limit(Some(pagination.page_size))
            .offset(pagination.page_offset())
            .all(&self.0.connection)
            .await
            .map_err(|err| {
                error!(
                    debug_error = ?err,
                    display_error = %err,
                    "failure, Unable to query user message by range with error"
                );

                RepoError::SomethingWentWrong
            })?;

        let count = PlatformUserMessageEntity::find()
            .count(&self.0.connection)
            .await
            .map_err(|err| {
                error!(
                    debug_error = ?err,
                    display_error = %err,
                    "failure, Unable to count user message by range"
                );

                RepoError::SomethingWentWrong
            })?;

        let result: UserMessages = result.into();

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

pub struct QueryUserMessage {
    pub platform_id: Uuid,
    pub is_dm: bool,
    pub user_id: Uuid,
}
