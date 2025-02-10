use {
    crate::connections::db::DB,
    async_trait::async_trait,
    sdk::{
        errors::RepoError,
        models::{
            db::extras::profile_media::{
                ActiveModel, Column, Entity as ChatMediaEntity,
                Model as ChatMedia,
            },
            others::extras::CreateChatMedia,
        },
    },
    sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter},
    tracing::{debug, error},
    uuid::Uuid,
};

#[async_trait]
pub trait ChatMediaRepository {
    async fn create(
        &self,
        payload: CreateChatMedia,
        request_id: Uuid,
    ) -> Result<ChatMedia, RepoError>;

    async fn get_by_message_id(
        &self,
        message_id: Uuid,
        request_id: Uuid,
    ) -> Result<ChatMedia, RepoError>;
}

pub struct ChatMediaRepo(DB);

impl ChatMediaRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl ChatMediaRepository for ChatMediaRepo {
    #[tracing::instrument(name = "ChatMediaRepo::create", skip(self))]
    async fn create(
        &self,
        payload: CreateChatMedia,
        request_id: Uuid,
    ) -> Result<ChatMedia, RepoError> {
        debug!("Received request to create new user chat_media, payload: {:?}, request_id:{request_id}", payload);

        let chat_media: ActiveModel = payload.into();

        let result =
            chat_media.insert(&self.0.connection).await.map_err(|err| {
                error!("Failed to insert profile into database: {}", err);

                RepoError::FailedToInsert
            })?;

        Ok(result)
    }

    #[tracing::instrument(
        name = "ChatMediaRepo::get_by_message_id",
        skip(self)
    )]
    async fn get_by_message_id(
        &self,
        message_id: Uuid,
        request_id: Uuid,
    ) -> Result<(), RepoError> {
        debug!("Received request to get a chat media by message, payload: {:?}, request_id:{request_id}", message_id);

        let chat_media = ChatMediaEntity::find()
            .filter(Column::MessageId.eq(message_id))
            .one(&self.0.connection)
            .await
            .map_err(|err| {
                error!("Failed to fetch profile from the database: {}", err);

                RepoError::NotFound
            })?
            .ok_or_else(|| {
                error!("No profile found for message id");
                RepoError::NotFound
            });

        chat_media
    }
}
