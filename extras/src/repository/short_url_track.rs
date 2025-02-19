use {
    crate::connections::db::DB,
    async_trait::async_trait,
    sdk::{
        errors::{RepoError, RepoResult},
        models::{
            db::extras::short_url_track::{
                ActiveModel,
                Model as ShortUrlTrack,
            },
            others::extras::CreateShortUrlTrack,
        },
    },
    sea_orm::{
        ActiveModelTrait, DbErr,
    },
    tracing::{debug, error},
    uuid::Uuid,
};

#[async_trait]
pub trait ShortUrlTrackRepository {
    async fn create(
        &self,
        payload: CreateShortUrlTrack,
        request_id: Uuid,
    ) -> RepoResult<ShortUrlTrack>;

    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<ShortUrlTrack>;
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

    async fn get_by_id(&self, _id: Uuid, _request_id: Uuid) -> RepoResult<ShortUrlTrack> {
        todo!()
    }
}
