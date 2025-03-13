use chrono::Utc;
use sea_orm::Set;
use {
    crate::connections::db::DB,
    async_trait::async_trait,
    sdk::{
        errors::{RepoError, RepoResult},
        models::{
            db::messaging::reaction::{
                ActiveModel, Entity as ReactionEntity, Model as Reaction,
            },
            others::messaging::CreateReaction,
        },
    },
    sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait},
    tracing::{debug, error},
    uuid::Uuid,
};

#[async_trait]
pub trait ReactionRepository {
    async fn create(
        &self,
        payload: CreateReaction,
        request_id: Uuid,
    ) -> RepoResult<Reaction>;

    async fn delete(&self, id: Uuid, request_id: Uuid) -> RepoResult<()>;

    async fn find_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<Reaction>;
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
    async fn create(
        &self,
        payload: CreateReaction,
        request_id: Uuid,
    ) -> RepoResult<Reaction> {
        debug!(
            request_id = %request_id,
            "Got request to create reaction"
        );

        let model: ActiveModel = payload.into();

        let result = model.insert(&self.0.connection).await.map_err(|err| {
            error!(
                debug_error = ?err,
                display_error = %err,
                "failure, unable to insert reaction"
            );

            RepoError::FailedToInsert
        })?;

        Ok(result)
    }

    #[tracing::instrument(skip(self), name = "ReactionRepository::delete")]
    async fn delete(&self, mut reaction: ActiveModel, request_id: Uuid) -> RepoResult<()> {
        debug!(
            request_id = %request_id
            "Got request to delete reaction"
        );

        reaction.deleted_at = Set(Some(Utc::now()));

        let _ = reaction.delete(&self.0.connection).await.map_err(|err| {
            error!(
                debug_error = ?err,
                display_error = %err,
                "failure, unable to delete reaction"
            );

            RepoError::SomethingWentWrong
        })?;

        Ok(())
    }

    #[tracing::instrument(skip(self), name = "ReactionRepository::find_by_id")]
    async fn find_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<Reaction> {
        debug!(
            request_id = %request_id,
            "Got request to find reaction by id"
        );

        let result = ReactionEntity::find_by_id(id)
            .one(&self.0.connection)
            .await
            .map_err(|err| {
                error!(
                    debug_error = ?err,
                    display_error = %err,
                    "failure, unable to find reaction"
                );

                RepoError::NotFound
            })?;

        if result.is_none() {
            error!("unable to find reaction");
            return Err(RepoError::NotFound);
        }

        Ok(result.unwrap())
    }
}
