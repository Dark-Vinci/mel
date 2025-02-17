use sdk::errors::RepoResult;
use {
    crate::{connections::db::DB, repository::channel_user},
    async_trait::async_trait,
    chrono::Utc,
    sdk::{
        errors::RepoError,
        models::{
            db::channel::{
                bookmarks::{
                    ActiveModel, Column, Entity as BookMarkEntity,
                    Model as BookMark,
                },
                // user,
            },
            others::{
                channel::{CreateBookMark, UpdateBookMark},
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
pub trait BookMarkRepository {
    async fn create(
        &self,
        payload: CreateBookMark,
        request_id: Uuid,
    ) -> RepoResult<BookMark>;

    async fn update(
        &self,
        payload: UpdateBookMark,
        request_id: Uuid,
    ) -> RepoResult<BookMark>;

    async fn delete(&self, id: Uuid, request_id: Uuid) -> RepoResult<()>;

    async fn get(
        &self,
        id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> RepoResult<Paginated<Vec<BookMark>>>;

    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<BookMark>;
}

pub struct BookMarkRepo(DB);

impl BookMarkRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl BookMarkRepository for BookMarkRepo {
    #[tracing::instrument(name = "BookMarkRepo::create", skip(self))]
    async fn create(
        &self,
        payload: CreateBookMark,
        request_id: Uuid,
    ) -> RepoResult<BookMark> {
        debug!(
            "Creating channel_user by id: {:?}, request_id {}",
            payload, request_id
        );

        let model: ActiveModel = payload.into();

        let result = model.insert(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to create channel_user");
            return Err(RepoError::FailedToUpdate);
        }

        Ok(result.unwrap())
    }

    #[tracing::instrument(name = "BookMarkRepo::update", skip(self))]
    async fn update(
        &self,
        payload: UpdateBookMark,
        request_id: Uuid,
    ) -> RepoResult<BookMark> {
        debug!(
            "Updating channel_user by id: {:?}, request_id {}",
            payload, request_id
        );

        let model: ActiveModel = payload.into();

        let result = model.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to update channel_user");
            return Err(RepoError::FailedToUpdate);
        }

        Ok(result.unwrap())
    }

    #[tracing::instrument(name = "BookMarkRepo::delete", skip(self))]
    async fn delete(&self, id: Uuid, request_id: Uuid) -> RepoResult<()> {
        debug!(
            "Deleting channel_user by id: {}, request_id {}",
            id, request_id
        );

        let mut channel_user =
            self.get_by_id(id, request_id).await?.into_active_model();

        channel_user.deleted_at = Set(Some(Utc::now()));

        let res = channel_user.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &res {
            error!(
                error = &err.to_string(),
                "Failed to find channel_user by mail"
            );
            return Err(RepoError::SomethingWentWrong);
        }

        Ok(())
    }

    #[tracing::instrument(name = "BookMarkRepo::get", skip(self))]
    async fn get(
        &self,
        id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> RepoResult<Paginated<Vec<BookMark>>> {
        debug!(
            "getting channel users by id: {}, request_id {}",
            id, request_id
        );

        let result = BookMarkEntity::find()
            .limit(Some(pagination.page_size)) // Set limit
            .offset(pagination.page_offset()) // Set offset
            .all(&self.0.connection) // Execute query
            .await;

        if let Err(DbErr::Query(err)) = &result {
            error!(
                error = &err.to_string(),
                "Failed to find channel_user by id"
            );
            return Err(RepoError::SomethingWentWrong);
        }

        let count = BookMarkEntity::find().count(&self.0.connection).await;

        if let Err(DbErr::Query(err)) = &count {
            error!(
                error = &err.to_string(),
                "Failed to find channel_user by id"
            );
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

    #[tracing::instrument(name = "BookMarkRepo::get_by_id", skip(self))]
    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<BookMark> {
        debug!(
            "Getting channel_user by id: {}, with request id: {}",
            id, request_id
        );

        let result =
            BookMarkEntity::find_by_id(id).one(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(
                error = &err.to_string(),
                "Failed to find channel_user by id"
            );
            return Err(RepoError::SomethingWentWrong);
        }

        let result = result.unwrap();

        if result.is_none() {
            error!("channel_user not found");
            return Err(RepoError::NotFound);
        }

        Ok(result.unwrap())
    }
}
