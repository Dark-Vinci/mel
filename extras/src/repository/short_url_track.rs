use crate::connections::db::DB;
use async_trait::async_trait;
use chrono::Utc;
use sdk::errors::RepoError;
use sdk::models::db::extras::short_url::{
    ActiveModel, Entity as ShortUrlTrackEntity, Model as ShortUrlTrack,
};
use sdk::models::others::extras::CreateShortUrlTrack;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, IntoActiveModel};
use tracing::{debug, error};
use uuid::Uuid;

#[async_trait]
pub trait ShortUrlTrackRepository {
    async fn create(
        &self,
        payload: CreateShortUrlTrack,
        request_id: Uuid,
    ) -> Result<ShortUrlTrack, RepoError>;
    async fn delete(&self, id: Uuid, request_id: Uuid)
        -> Result<(), RepoError>;
}

pub struct ShortUrlTrackRepo(DB);

impl ShortUrlTrackRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl ShortUrlTrackRepository for ShortUrlTrackRepo {
    #[tracing::instrument(name = "ShortUrlTrackRepo::create", skip(self))]
    async fn create(
        &self,
        payload: CreateShortUrlTrack,
        request_id: Uuid,
    ) -> Result<ShortUrlTrack, RepoError> {
        debug!("ShortUrlTrackRepo::create called, payload: {:?}, request_id: {request_id}", payload);

        let short_url: ActiveModel = payload.into();

        let result =
            short_url.insert(&self.0.connection).await.map_err(|err| {
                error!("Failed to insert shortUrl into database: {}", err);

                if err == DbErr::RecordNotInserted {
                    return RepoError::FailedToInsert;
                }

                return RepoError::SomethingWentWrong;
            })?;

        Ok(result)
    }

    #[tracing::instrument(name = "ShortUrlTrackRepo::delete", skip(self))]
    async fn delete(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<(), RepoError> {
        debug!("ShortUrlTrackRepo::delete called, id: {}, request_id: {request_id}", id);

        let mut result =
            self.get_by_id(request_id, id).await?.into_active_model();

        result.deleted_at = Set(Some(Utc::now()));

        let _ = result.update(&self.0.connection).await.map_err(|err| {
            return RepoError::SomethingWentWrong;
        })?;

        Ok(())
    }
}
