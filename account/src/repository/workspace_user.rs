use {
    crate::{connections::db::DB, repository::workspace_user},
    async_trait::async_trait,
    chrono::Utc,
    sdk::{
        errors::RepoError,
        models::{
            db::account::{
                user,
                workspace_user::{
                    ActiveModel, Column, Entity as WorkspaceUserEntity,
                    Model as WorkspaceUser,
                },
            },
            others::{
                auth::workspace::{CreateWorkspaceUser, UpdateWorkspaceUser},
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
pub trait WorkspaceUserRepository {
    async fn create(
        &self,
        payload: CreateWorkspaceUser,
        request_id: Uuid,
    ) -> Result<WorkspaceUser, RepoError>;

    async fn update(
        &self,
        payload: UpdateWorkspaceUser,
        request_id: Uuid,
    ) -> Result<WorkspaceUser, RepoError>;

    async fn delete(&self, id: Uuid, request_id: Uuid)
                    -> Result<(), RepoError>;

    async fn get(
        &self,
        id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> Result<Paginated<Vec<WorkspaceUser>>, RepoError>;

    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<WorkspaceUser, RepoError>;
}

pub struct WorkspaceUserRepo(DB);

impl WorkspaceUserRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl WorkspaceUserRepository for WorkspaceUserRepo {
    #[tracing::instrument(name = "WorkspaceUserRepo::create", skip(self))]
    async fn create(
        &self,
        payload: CreateWorkspaceUser,
        request_id: Uuid,
    ) -> Result<WorkspaceUser, RepoError> {
        debug!(
            "Creating workspace_user by id: {:?}, request_id {}",
            payload, request_id
        );

        let model: ActiveModel = payload.into();

        let result = model.insert(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to create workspace_user");
            return Err(RepoError::FailedToUpdate);
        }

        Ok(result.unwrap())
    }

    #[tracing::instrument(name = "WorkspaceUserRepo::update", skip(self))]
    async fn update(
        &self,
        payload: UpdateWorkspaceUser,
        request_id: Uuid,
    ) -> Result<WorkspaceUser, RepoError> {
        debug!(
            "Updating workspace_user by id: {:?}, request_id {}",
            payload, request_id
        );

        let model: ActiveModel = payload.into();

        let result = model.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to update workspace_user");
            return Err(RepoError::FailedToUpdate);
        }

        Ok(result.unwrap())
    }

    #[tracing::instrument(name = "WorkspaceUserRepo::delete", skip(self))]
    async fn delete(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<(), RepoError> {
        debug!(
            "Deleting workspace_user by id: {}, request_id {}",
            id, request_id
        );

        let mut workspace_user =
            self.get_by_id(request_id, id).await?.into_active_model();

        workspace_user.deleted_at = Set(Some(Utc::now()));

        let res = workspace_user.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &res {
            error!(
                error = &err.to_string(),
                "Failed to find workspace_user by mail"
            );
            return Err(RepoError::SomethingWentWrong);
        }

        Ok(())
    }

    #[tracing::instrument(name = "WorkspaceUserRepo::get", skip(self))]
    async fn get(
        &self,
        id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> Result<Paginated<Vec<WorkspaceUser>>, RepoError> {
        debug!(
            "getting channel users by id: {}, request_id {}",
            id, request_id
        );

        let result = WorkspaceUserEntity::find()
            .limit(Some(pagination.page_size)) // Set limit
            .offset((pagination.page_number - 1) * pagination.page_size) // Set offset
            .all(&self.0.connection) // Execute query
            .await;

        if let Err(DbErr::Query(err)) = &result {
            error!(
                error = &err.to_string(),
                "Failed to find workspace_user by id"
            );
            return Err(RepoError::SomethingWentWrong);
        }

        let count = WorkspaceUserEntity::find().count(&self.0.connection).await;

        if let Err(DbErr::Query(err)) = &count {
            error!(
                error = &err.to_string(),
                "Failed to find workspace_user by id"
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

    #[tracing::instrument(name = "WorkspaceUserRepo::get_by_id", skip(self))]
    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<WorkspaceUser, RepoError> {
        debug!(
            "Getting workspace_user by id: {}, with request id: {}",
            id, request_id
        );

        let result = WorkspaceUserEntity::find_by_id(id)
            .one(&self.0.connection)
            .await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(
                error = &err.to_string(),
                "Failed to find workspace_user by id"
            );
            return Err(RepoError::SomethingWentWrong);
        }

        let result = result.unwrap();

        if result.is_none() {
            error!("workspace_user not found");
            return Err(RepoError::NotFound);
        }

        Ok(result.unwrap())
    }
}
