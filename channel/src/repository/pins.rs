use {
    crate::connections::db::DB,
    async_trait::async_trait,
    chrono::Utc,
    sdk::{
        errors::RepoError,
        models::{
            db::channel::channel::{
                ActiveModel, Column, Entity as PinEntity, Model as Pin,
            },
            others::{
                channel::{CreatePin, UpdatePin},
                Paginated, Pagination,
            },
        },
    },
    sea_orm::{
        entity::*, query::*, ActiveModelTrait, ActiveValue::Set, ColumnTrait,
        Condition, DbErr, EntityTrait, IntoActiveModel,
    },
    tracing::{debug, error},
    uuid::Uuid,
};

#[async_trait]
pub trait PinRepository {
    async fn create(
        &self,
        payload: CreatePin,
        request_id: Uuid,
    ) -> Result<Pin, RepoError>;

    async fn update(
        &self,
        payload: UpdatePin,
        request_id: Uuid,
    ) -> Result<Pin, RepoError>;

    async fn delete(&self, id: Uuid, request_id: Uuid)
        -> Result<(), RepoError>;

    async fn get(
        &self,
        id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> Result<Paginated<Vec<Pin>>, RepoError>;

    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<Pin, RepoError>;
}

pub struct PinRepo(DB);

impl PinRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl PinRepository for PinRepo {
    #[tracing::instrument(name = "PinRepo::create", skip(self))]
    async fn create(
        &self,
        payload: CreatePin,
        request_id: Uuid,
    ) -> Result<Pin, RepoError> {
        debug!("Creating pin by id: {:?}, request_id {}", payload, request_id);

        let model: ActiveModel = payload.into();

        let result = model.insert(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to create pin");
            return Err(RepoError::FailedToUpdate);
        }

        Ok(result.unwrap())
    }

    #[tracing::instrument(name = "PinRepo::update", skip(self))]
    async fn update(
        &self,
        payload: UpdatePin,
        request_id: Uuid,
    ) -> Result<Pin, RepoError> {
        debug!("Updating pin by id: {:?}, request_id {}", payload, request_id);

        let model: ActiveModel = payload.into();

        let result = model.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to update pin");
            return Err(RepoError::FailedToUpdate);
        }

        Ok(result.unwrap())
    }

    #[tracing::instrument(name = "PinRepo::delete", skip(self))]
    async fn delete(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<(), RepoError> {
        debug!("Deleting pin by id: {}, request_id {}", id, request_id);

        let mut pin = self.get_by_id(id, request_id).await?.into_active_model();

        pin.deleted_at = Set(Some(Utc::now()));

        let res = pin.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &res {
            error!(error = &err.to_string(), "Failed to find pin by mail");
            return Err(RepoError::SomethingWentWrong);
        }

        Ok(())
    }

    #[tracing::instrument(name = "PinRepo::get", skip(self))]
    async fn get(
        &self,
        id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> Result<Paginated<Vec<Pin>>, RepoError> {
        debug!("getting pin users by id: {}, request_id {}", id, request_id);

        let result = PinEntity::find()
            .limit(Some(pagination.page_size)) // Set limit
            .offset((pagination.page_number - 1) * pagination.page_size) // Set offset
            .all(&self.0.connection) // Execute query
            .await;

        if let Err(DbErr::Query(err)) = &result {
            error!(error = &err.to_string(), "Failed to find pin by id");
            return Err(RepoError::SomethingWentWrong);
        }

        let count = PinEntity::find().count(&self.0.connection).await;

        if let Err(DbErr::Query(err)) = &count {
            error!(error = &err.to_string(), "Failed to find pin by id");
            return Err(RepoError::SomethingWentWrong);
        }

        let count = count.unwrap();

        let total_pages =
            (count + pagination.page_size - 1) / pagination.page_size;

        let paginated = Paginated {
            result: result.unwrap(),
            total_pages,
            current_page: 0,
            page_size: 0,
            total_items: count,
        };

        Ok(paginated)
    }

    #[tracing::instrument(name = "PinRepo::get_by_id", skip(self))]
    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<Pin, RepoError> {
        debug!("Getting pin by id: {}, with request id: {}", id, request_id);

        let result = PinEntity::find_by_id(id).one(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to find pin by id");
            return Err(RepoError::SomethingWentWrong);
        }

        let result = result.unwrap();

        if result.is_none() {
            error!("pin not found");
            return Err(RepoError::NotFound);
        }

        Ok(result.unwrap())
    }
}
