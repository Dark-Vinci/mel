use {
    crate::connections::db::DB,
    async_trait::async_trait,
    sdk::{
        errors::{RepoError, RepoResult},
        models::{
            db::extras::{
                emails::{ActiveModel, Entity as EmailEntity, Model as Email},
                search::Column,
            },
            others::{
                extras::{CreateEmail, CreateHistory, CreateShortUrl},
                Paginated, Pagination,
            },
        },
    },
    sea_orm::{ActiveModelTrait, DbErr},
    tracing::{debug, error},
    uuid::Uuid,
};

#[async_trait]
pub trait EmailRepository {
    async fn create(
        &self,
        payload: CreateEmail,
        request_id: Uuid,
    ) -> RepoResult<Email>;

    async fn update(
        &self,
        payload: ActiveModel,
        request_id: Uuid,
    ) -> RepoResult<()>;
}

pub struct EmailRepo(DB);

impl EmailRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl EmailRepository for EmailRepo {
    #[tracing::instrument(name = "EmailRepo::create", skip(self))]
    async fn create(
        &self,
        payload: CreateEmail,
        request_id: Uuid,
    ) -> RepoResult<Email> {
        debug!(
            request_id = ?request_id
            "Got a request to create email detail"
        );

        let email: ActiveModel = payload.into();

        let result = email.insert(&self.0.connection).await.map_err(|err| {
            error!(
                display = %err,
                debug = ?err,
                "unable to insert email into database"
            );

            if err == DbErr::RecordNotInserted {
                return RepoError::FailedToInsert;
            }

            return RepoError::SomethingWentWrong;
        })?;

        Ok(result)
    }

    #[tracing::instrument(name = "EmailRepo::update", skip(self))]
    async fn update(
        &self,
        mut payload: ActiveModel,
        request_id: Uuid,
    ) -> RepoResult<()> {
        debug!(
            request_id = request_id
            "Got a request to update the email detail payload"
        );

        payload.set_seen();

        let _ = payload.save(&self.0.connection).await.map_err(|err| {
            error!(
                display = %err,
                debug = ?err,
                "Unable to update email details"
            );

            return RepoError::FailedToUpdate;
        })?;

        Ok(())
    }
}
