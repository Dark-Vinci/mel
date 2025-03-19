use {
    crate::connections::db::DB,
    async_trait::async_trait,
    sdk::{
        errors::RepoError,
        models::{
            db::messaging::chat::{
                ActiveModel, Column, Entity as ChatEntity, Model as Chat,
            },
            others::{
                messaging::{CreateChat},
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
use sdk::errors::RepoResult;

#[async_trait]
pub trait ChatRepository {
    async fn create(
        &self,
        payload: CreateChat,
        request_id: Uuid,
    ) -> RepoResult<Chat>;

    async fn find_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<Chat>;

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
        debug!("Got request to create direct message(chat), payload: {}, request_id: {}", payload, request_id);

        let chat: ActiveModel = payload.into();

        let result = chat.insert(&self.0.connection).await.map_err(|err| {
            error!("Failed to insert into database: {}", err);

            RepoError::FailedToInsert
        })?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<Chat> {
        debug!(
            "Got request to find direct message(id: {}), request_id: {}",
            id, request_id
        );

        let chat = ChatEntity::find_by_id(id)
            .one(&self.0.connection)
            .await
            .map_err(|err| {
                error!("failure, unable to find chat by id");

                RepoError::NotFound
            })?;

        if chat.is_none() {
            error!("Chat not found");

            return Err(RepoError::NotFound);
        }

        Ok(chat.unwrap())
    }

    async fn find_for_user(
        &self,
        user_id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> RepoResult<Paginated<Vec<Chat>>> {
        debug!(
            "Got request to find for chat(user_id: {}, request_id: {})",
            user_id, request_id
        );

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
                error!("Unable to fetch user chats");

                RepoError::SomethingWentWrong
            })?;

        let count = ChatEntity::find().count().await.map_err(|err| {
            error!("Unable to count message by range with error {err}");

            RepoError::SomethingWentWrong
        })?;

        let paginated =
            Paginated::new(result, pagination.total_pages(count), 0, 0, count);

        Ok(paginated)
    }
}
