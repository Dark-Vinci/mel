use {
    crate::connections::db::DB,
    async_trait::async_trait,
    chrono::Utc,
    sdk::{
        errors::{RepoError, RepoResult},
        models::{
            db::messaging::{
                message::{Entity as MessageEntity, Model as Message},
                platform_user_message::{
                    ActiveModel, Column, Entity as PlatformUserMessageEntity,
                    Model as PlatformUserMessage,
                },
            },
            others::{
                messaging::{
                    CreateChat, CreatePlatformUserMessage,
                    QueryUserMessagePayload, UpdateChat,
                    UpdatePlatformUserMessage, UserMessages,
                },
                Paginated, Pagination,
            },
        },
    },
    sea_orm::{
        tests_cfg::cake, ActiveModelTrait, ActiveValue::Set, Condition, DbErr,
        EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, QuerySelect,
    },
    tracing::{debug, error},
    uuid::Uuid,
};

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
        payload: QueryUserMessagePayload,
        pagination: Pagination,
        request_id: Uuid,
    ) -> RepoResult<Paginated<UserMessages>>;
}

pub struct PlatformUserMessageRepo(DB);

impl PlatformUserMessageRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl PlatformUserMessageRepository for PlatformUserMessageRepo {
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

        let result =
            user_message
                .insert(&self.0.connection)
                .await
                .map_err(|err| {
                    error!(
                        debug_error = ?err,
                        display_error = %err,
                        "Failed to insert into database"
                    );

                    RepoError::FailedToInsert
                })?;

        Ok(result)
    }

    #[tracing::instrument(
        skip(self),
        name = "PlatformUserMessageRepository::update"
    )]
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

    #[tracing::instrument(
        skip(self),
        name = "PlatformUserMessageRepository::delete"
    )]
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

    #[tracing::instrument(
        skip(self),
        name = "PlatformUserMessageRepository::get_platform_message"
    )]
    async fn get_platform_message(
        &self,
        payload: QueryUserMessagePayload,
        pagination: Pagination,
        request_id: Uuid,
    ) -> RepoResult<Paginated<UserMessages>> {
        debug!(
            request_id = %request_id,
            "Got request to get platform messages"
        );

        let (result, count) = tokio::join!(
            PlatformUserMessageEntity::find()
                .find_also_related(MessageEntity)
                .filter(
                    Condition::all()
                        .add(Column::PlatformId.eq(payload.platform_id))
                        .add(Column::UserId.eq(payload.user_id))
                        .add(Column::IsPrivateMessage.eq(payload.is_dm))
                )
                .limit(Some(pagination.page_size))
                .offset(pagination.page_offset())
                .all(&self.0.connection),
            PlatformUserMessageEntity::find()
                .filter(
                    Condition::all()
                        .add(Column::PlatformId.eq(payload.platform_id))
                        .add(Column::UserId.eq(payload.user_id))
                        .add(Column::IsPrivateMessage.eq(payload.is_dm))
                )
                .count(&self.0.connection)
        );

        let unwrapped_result = result.map_err(|err| {
            error!(
                debug_error = ?err,
                display_error = %err,
                "failure, Unable to query user message by range with error"
            );

            RepoError::SomethingWentWrong
        })?;

        let unwrapped_count = count.map_err(|err| {
            error!(
                debug_error = ?err,
                display_error = %err,
                "failure, Unable to count user message by range"
            );

            RepoError::SomethingWentWrong
        })?;

        let formatted_result: UserMessages = unwrapped_result.into();

        let paginated = Paginated::new(
            formatted_result,
            pagination.total_pages(unwrapped_count),
            pagination.page_number + 1,
            pagination.page_size,
            unwrapped_count,
        );

        Ok(paginated)
    }
}
