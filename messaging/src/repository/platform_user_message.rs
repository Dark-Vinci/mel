use sea_orm::ActiveModelTrait;
use tracing::{debug, error};
// use sdk::errors::RepoResult;
use sdk::{
    errors::{RepoError, RepoResult},
    models::{
        db::messaging::message::Model as Message,
        db::messaging::platform_user_message::{
            ActiveModel, Column, Entity as PlatformUserMessageEntity,
            Model as PlatformUserMessage,
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
        request_id: Uuid,
    ) -> RepoResult<PlatformUserMessage>;

    async fn delete(
        &self,
        message: ActiveModel,
        request_id: Uuid,
    ) -> RepoResult<()>;

    async fn get_platform_message(
        &self,
        platform_id: Uuid,
        is_channel: bool,
        user_id: Uuid,
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

    async fn update(
        &self,
        payload: UpdatePlatformUserMessage,
        request_id: Uuid,
    ) -> RepoResult<PlatformUserMessage> {
        todo!()
    }

    async fn delete(
        &self,
        message: ActiveModel,
        request_id: Uuid,
    ) -> RepoResult<()> {
        todo!()
    }

    async fn get_platform_message(
        &self,
        platform_id: Uuid,
        is_channel: bool,
        user_id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<Paginated<UserMessages>> {
        todo!()
    }
}
