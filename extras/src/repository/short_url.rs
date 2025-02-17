use {
    crate::connections::db::DB,
    async_trait::async_trait,
    chrono::Utc,
    sdk::{
        errors::RepoError,
        models::{
            db::extras::short_url::{
                ActiveModel, Entity as ShortUrlEntity, Model as ShortUrl,
            },
            others::extras::CreateShortUrl,
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
pub trait ShortUrlRepository {
    async fn create(
        &self,
        payload: CreateShortUrl,
        request_id: Uuid,
    ) -> RepoResult<ShortUrl>;

    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<ShortUrl>;

    async fn delete(&self, id: Uuid, request_id: Uuid)
        -> RepoResult<()>;
}

pub struct ShortUrlRepo(DB);

impl ShortUrlRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl ShortUrlRepository for ShortUrlRepo {
    #[tracing::instrument(name = "ShortUrlRepo::create", skip(self))]
    async fn create(
        &self,
        payload: CreateShortUrl,
        request_id: Uuid,
    ) -> RepoResult<ShortUrl> {
        debug!("ShortUrlRepo::create called, payload: {:?}, request_id: {request_id}", payload);

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

    #[tracing::instrument(name = "ShortUrlRepo::get_by_id", skip(self))]
    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<ShortUrl> {
        debug!(
            "ShortUrlRepo::get_by_id called, id: {}, request_id: {request_id}",
            id
        );

        let short_ur = ShortUrlEntity::find_by_id(id)
            .one(&self.0.connection)
            .await
            .map_err(|err| {
                return RepoError::NotFound;
            })?;

        if short_ur.is_none() {
            return Err(RepoError::NotFound);
        }

        Ok(short_ur.unwrap())
    }

    #[tracing::instrument(name = "ShortUrlRepo::delete", skip(self))]
    async fn delete(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<()> {
        debug!(
            "ShortUrlRepo::delete called, id: {}, request_id: {request_id}",
            id
        );

        let mut result =
            self.get_by_id(request_id, id).await?.into_active_model();

        result.deleted_at = Set(Some(Utc::now()));

        let _ = result.update(&self.0.connection).await.map_err(|err| {
            return RepoError::SomethingWentWrong;
        })?;

        Ok(())
    }
}
