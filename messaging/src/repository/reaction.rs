use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait};
use tracing::{debug, error};
use uuid::Uuid;
use sdk::errors::RepoError;
use sdk::models::db::messaging::reaction::{ActiveModel, Model as Reaction, Entity as ReactionEntity};
use sdk::models::others::messaging::CreateReaction;
use crate::connections::db::DB;

#[async_trait]
pub trait ReactionRepository {
    async fn create(&self, payload: CreateReaction, request_id: Uuid) -> Result<Reaction, RepoError>;

    async fn delete(&self, id: Uuid, request_id: Uuid) -> Result<(), RepoError>;

    async fn find_by_id(&self, id: Uuid, request_id: Uuid) -> Result<Reaction, RepoError>;
}

pub struct ReactionRepo(DB);

impl ReactionRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl ReactionRepository for ReactionRepo {
    #[tracing::instrument(skip(self), name = "ReactionRepository::create")]
    async fn create(&self, payload: CreateReaction, request_id: Uuid) -> Result<Reaction, RepoError> {
        debug!("Got request to create reaction with payload {}, request_id: {}", payload, request_id);

        let model : ActiveModel = payload.into();

        let result = model.insert(&self.0.connection).await.map_err(|err| {
            error!("Failed to insert reaction: {:?}", err);

            RepoError::FailedToInsert
        })?;

        Ok(result)
    }

    #[tracing::instrument(skip(self), name = "ReactionRepository::delete")]
    async fn delete(&self, id: Uuid, request_id: Uuid) -> Result<(), RepoError> {
        debug!("Got request to delete reaction with id {} and request_id {}", id, request_id);

        let reaction = self.find_by_id(request_id, id).await.map_err(|err| {
            error!("Failed to find reaction: {:?}", err);

            RepoError::NotFound
        })?;

        let _ = reaction.delete(&self.0.connection).await.map_err(|err| {
            error!("Failed to delete reaction: {:?}", err);

            RepoError::SomethingWentWrong
        })?;

        Ok(())
    }

    #[tracing::instrument(skip(self), name = "ReactionRepository::find_by_id")]
    async fn find_by_id(&self, id: Uuid, request_id: Uuid) -> Result<Reaction, RepoError> {
        debug!("Got request to find reaction with id {} with request_id {}", id, request_id);

        let result = ReactionEntity::find_by_id(id).one(&self.0.connection).await.map_err(|err| {
            error!("Failed to find reaction: {:?}", err);

            RepoError::NotFound
        })?;

        if result.is_none() {
            return Err(RepoError::NotFound);
        }

        Ok(result.unwrap())
    }
}