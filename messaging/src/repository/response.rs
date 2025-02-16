use {
    crate::connections::db::DB,
    async_trait::async_trait,
    chrono::Utc,
    sdk::{
        errors::RepoError,
        models::{
            db::messaging::response::{
                ActiveModel, Entity as ResponseEntity, Model as Response,
            },
            others::messaging::{CreateResponse, UpdateResponse},
        },
    },
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, EntityTrait, IntoActiveModel,
    },
    tracing::{debug, error},
    uuid::Uuid,
};

#[async_trait]
pub trait ResponseRepository {
    async fn create(
        &self,
        payload: CreateResponse,
        request_id: Uuid,
    ) -> Result<Response, RepoError>;

    async fn update(
        &self,
        payload: UpdateResponse,
        request_id: Uuid,
    ) -> Result<Response, RepoError>;

    async fn delete(&self, id: Uuid, request_id: Uuid)
        -> Result<(), RepoError>;

    async fn find_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<Response, RepoError>;
}

pub struct ResponseRepo(DB);

impl ResponseRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl ResponseRepository for ResponseRepo {
    #[tracing::instrument(skip(self), name = "ResponseRepository::create")]
    async fn create(
        &self,
        payload: CreateResponse,
        request_id: Uuid,
    ) -> Result<Response, RepoError> {
        debug!(
            "Got a create response request with payload: {}, request_id: {}",
            payload, request_id
        );

        let model: ActiveModel = payload.into();

        let result = model.insert(&self.0.connection).await.map_err(|err| {
            error!("Failed to insert into database: {}", err);

            RepoError::FailedToInsert
        })?;

        Ok(result)
    }

    #[tracing::instrument(skip(self), name = "ResponseRepository::update")]
    async fn update(
        &self,
        payload: UpdateResponse,
        request_id: Uuid,
    ) -> Result<Response, RepoError> {
        debug!(
            "Got a update request with payload: {}, request_id: {}",
            payload, request_id
        );

        let model: ActiveModel = payload.into();

        let result = model.update(&self.0.connection).await.map_err(|err| {
            error!("Failed to update into database: {}", err);

            RepoError::FailedToUpdate
        })?;

        Ok(result)
    }

    #[tracing::instrument(skip(self), name = "ResponseRepository::delete")]
    async fn delete(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<(), RepoError> {
        debug!(
            "Got delete request with id: {}, request_id: {}",
            id, request_id
        );

        let mut response =
            self.find_by_id(id, request_id).await?.into_active_model();

        response.deleted_at = Set(Some(Utc::now()));

        let _ = response.update(&self.0.connection).await.map_err(|err| {
            error!("Failed to delete into database: {}", err);

            RepoError::FailedToUpdate
        })?;

        Ok(())
    }

    #[tracing::instrument(skip(self), name = "ResponseRepository::find_by_id")]
    async fn find_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<Response, RepoError> {
        debug!("Got find request with id: {}, request_id: {}", id, request_id);

        let response = ResponseEntity::find_by_id(id)
            .one(&self.0.connection)
            .await
            .map_err(|err| {
                error!("Failed to find into entity: {}", err);

                RepoError::NotFound
            })?;

        if response.is_none() {
            error!("Response not found");
            return Err(RepoError::NotFound);
        }

        Ok(response.unwrap())
    }
}
