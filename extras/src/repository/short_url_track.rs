use {
    crate::connections::db::DB,
    async_trait::async_trait,
    chrono::Utc,
    sdk::{
        errors::RepoError,
        models::{
            db::extras::short_url::{
                ActiveModel, Entity as ShortUrlTrackEntity,
                Model as ShortUrlTrack,
            },
            others::extras::CreateShortUrlTrack,
        },
    },
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait, IntoActiveModel,
    },
    tracing::{debug, error},
    uuid::Uuid,
};
use sdk::errors::RepoResult;

#[async_trait]
pub trait ShortUrlTrackRepository {
    async fn create(
        &self,
        payload: CreateShortUrlTrack,
        request_id: Uuid,
    ) -> RepoResult<ShortUrlTrack>;

    async fn delete(&self, id: Uuid, request_id: Uuid)
        -> RepoResult<()>;
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
    ) -> RepoResult<ShortUrlTrack> {
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
    ) -> RepoResult<()> {
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
