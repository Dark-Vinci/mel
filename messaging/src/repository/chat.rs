use {
    crate::connections::db::DB,
    async_trait::async_trait,
    sdk::{
        errors::{RepoError, RepoResult},
        models::{
            db::messaging::chat::{
                ActiveModel, Column, Entity as ChatEntity, Model as Chat,
            },
            others::{
                messaging::{CreateChat, UpdateChat},
                Paginated, Pagination,
            },
        },
    },
    sea_orm::{
        ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QuerySelect,
    },
    tonic::codegen::tokio_stream::StreamExt,
    tracing::{debug, error},
    uuid::Uuid,
};

#[async_trait]
pub trait ChatRepository {
    async fn create(
        &self,
        payload: CreateChat,
        request_id: Uuid,
    ) -> RepoResult<Chat>;

    async fn find_by_id(&self, id: Uuid, request_id: Uuid) -> RepoResult<Chat>;

    async fn find_for_user(
        &self,
        user_id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> Result<Paginated<Vec<Chat>>, RepoError>;
}

pub struct ChatRepo(DB);

impl ChatRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl ChatRepository for ChatRepo {
    #[tracing::instrument(skip(self), name = "ChatRepository::create")]
    async fn create(
        &self,
        payload: CreateChat,
        request_id: Uuid,
    ) -> RepoResult<Chat> {
        debug!(
            request_id = %request_id,
            "Got request to create direct chat(private message)"
        );

        let chat: ActiveModel = payload.into();

        let result = chat.insert(&self.0.connection).await.map_err(|err| {
            error!(
                debug_error = ?err,
                display_error = %err,
                "Failed to insert into database"
            );

            RepoError::FailedToInsert
        })?;

        Ok(result)
    }

    #[tracing::instrument(skip(self), name = "ChatRepository::find_by_id")]
    async fn find_by_id(&self, id: Uuid, request_id: Uuid) -> RepoResult<Chat> {
        debug!(
            request_id = %request_id,
            "Got request to find direct message",
        );

        let chat = ChatEntity::find_by_id(id)
            .one(&self.0.connection)
            .await
            .map_err(|err| {
                error!(
                    debug_error = ?err,
                    display_error = %err,
                    "failure, unable to find chat by id"
                );

                RepoError::NotFound
            })?;

        if chat.is_none() {
            error!("Chat not found");

            return Err(RepoError::NotFound);
        }

        Ok(chat.unwrap())
    }

    #[tracing::instrument(skip(self), name = "ChatRepository::find_for_user")]
    async fn find_for_user(
        &self,
        user_id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> RepoResult<Paginated<Vec<Chat>>> {
        debug!(
            request_id = %request_id,
            "Got request to fetch all user chats"
        );

        // todo; use tokio::join() for parallel fetch

        let result = ChatEntity::find()
            .filter(
                Condition::any()
                    .add(Column::UserA.eq(user_id))
                    .add(Column::UserB.eq(user_id)),
            )
            .limit(Some(pagination.page_size).into())
            .all(&self.0.connection)
            .await
            .map_err(|err| {
                error!(
                    debug_error = ?err,
                    display_error = %err,
                    "failure; unable to fetch all user chat"
                );

                RepoError::SomethingWentWrong
            })?;

        let count = ChatEntity::find().count().await.map_err(|err| {
            error!(
                debug_error = ?err,
                display_error = %err,
                "failure; unable to count all user chat"
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
