use crate::connections::db::DB;
use async_trait::async_trait;
use chrono::Utc;
use sdk::errors::RepoError;
use sdk::models::db::extras::profile_media::{
    ActiveModel, Entity as ProfileMediaEntity, Model as ProfileMedia,
};
use sdk::models::others::extras::CreateProfileMedia;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};
use sea_orm::ActiveValue::Set;
use tracing::{debug, error};
use uuid::Uuid;

#[async_trait]
pub trait ProfileMediaRepository {
    async fn create(
        &self,
        payload: CreateProfileMedia,
        request_id: Uuid,
    ) -> Result<ProfileMedia, RepoError>;

    async fn delete(
        &self,
        workspace_user_id: Uuid,
        request_id: Uuid,
    ) -> Result<(), RepoError>;

    async fn get_by_id(
        &self,
        workspace_user_id: Uuid,
        request_id: Uuid,
    ) -> Result<ProfileMedia, RepoError>;
}

pub struct ProfileMediaRepo(DB);

impl ProfileMediaRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

impl ProfileMediaRepository for ProfileMediaRepo {
    #[tracing::instrument(name = "ProfileMediaRepo::create", skip(self))]
    async fn create(
        &self,
        payload: CreateProfileMedia,
        request_id: Uuid,
    ) -> Result<ProfileMedia, RepoError> {
        debug!("Received request to create new user profile, payload: {:?}, request_id:{request_id}", payload);

        let profile: ActiveModel = payload.into();

        let result =
            profile.insert(&self.0.connection).await.map_err(|err| {
                error!("Failed to insert profile into database: {}", err);

                RepoError::FailedToInsert
            })?;

        Ok(result)
    }

    #[tracing::instrument(name = "ProfileMediaRepo::delete", skip(self))]
    async fn delete(
        &self,
        workspace_user_id: Uuid,
        request_id: Uuid,
    ) -> Result<(), RepoError> {
        debug!("Received request to delete profile, id: {:?}, request_id:{request_id}", workspace_user_id);

        let mut profile_media = self.get_by_id(workspace_user_id, request_id).await?.into_active_model();

        profile_media.deleted_at = Set(Some(Utc::now()));

        let _ = profile_media
            .save(&self.0.connection)
            .await
            .map_err(|err| {
                error!("Failed to delete profile from database: {}", err);

                return RepoError::FailedToUpdate;
            })?;

        Ok(())
    }

    #[tracing::instrument(name = "ProfileMediaRepo::get_by_id", skip(self))]
    async fn get_by_id(
        &self,
        workspace_user_id: Uuid,
        request_id: Uuid,
    ) -> Result<ProfileMedia, RepoError> {
        debug!("Received request to create new user profile, payload: {:?}, request_id:{request_id}", workspace_user_id);

        let result = ProfileMediaEntity::find_by_id(workspace_user_id)
            .one(&self.0.connection)
            .await
            .map_err(|err| {
                error!("Failed to insert profile into database: {}", err);

                RepoError::NotFound
            })?
            .ok_or_else(|| {
                error!("ProfileMediaRepository::get_by_id.return_value not found");

                return RepoError::NotFound;
            });

        result
    }
}
