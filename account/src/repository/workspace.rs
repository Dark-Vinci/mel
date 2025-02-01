use chrono::Utc;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, IntoActiveModel};
use sea_orm::ActiveValue::Set;
use tracing::{debug, error};
use uuid::Uuid;
use sdk::errors::RepoError;
use sdk::models::db::account::{user, workspace};
use sdk::models::db::account::workspace::Model as Workspace;
use sdk::models::db::account::workspace::Entity as WorkspaceEntity;
use sdk::models::others::auth::workspace::{CreateWorkspaceRequest, UpdateWorkspaceRequest};
use crate::connections::db::DB;

#[async_trait::async_trait]
pub trait WorkspaceRepository {
    async fn create(
        &self,
        result: CreateWorkspaceRequest,
        request_id: Uuid,
    ) -> Result<Workspace, RepoError>;

    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<Workspace, RepoError>;

    async fn delete(
        &self,
        request_id: Uuid,
        id: Uuid,
    ) -> Result<(), RepoError>;

    async fn update(
        &self,
        request_id: Uuid,
        result: UpdateWorkspaceRequest,
    ) -> Result<Workspace, RepoError>;
}

pub struct WorkspaceRepo(DB);

impl WorkspaceRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait::async_trait]
impl WorkspaceRepository for WorkspaceRepo {
    async fn create(&self, workspace: CreateWorkspaceRequest, request_id: Uuid) -> Result<Workspace, RepoError> {
        debug!("Creating workspace: {:?}, with request id: {}", workspace, request_id);

        let workActive: workspace::ActiveModel = workspace.into();

        let result = workActive.insert(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = result {
            error!(error = &err.to_string(), "Failed to insert result record");

            if err.to_string().contains("duplicate key") {
                return Err(RepoError::DuplicateKey);
            }

            return Err(RepoError::FailedToInsert);
        }

        Ok(result.unwrap())
    }

    async fn get_by_id(&self, id: Uuid, request_id: Uuid) -> Result<Workspace, RepoError> {
        debug!("Getting workspace by id: {}, with request id: {}", id, request_id);

        let result = WorkspaceEntity::find_by_id(id).one(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to find workspace by id");
            return Err(RepoError::SomethingWentWrong);
        }

        let result = result.unwrap();

        if result.is_none() {
            error!("workspace not found");
            return Err(RepoError::NotFound);
        }

        Ok(result.unwrap())
    }

    async fn delete(&self, request_id: Uuid, id: Uuid) -> Result<(), RepoError> {
        debug!("Deleting workspace by id: {}, request_id {}", id, request_id);

        let mut result =
            self.get_by_id(request_id, id).await?.into_active_model();

        result.deleted_at = Set(Some(Utc::now()));

        let res = result.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &res {
            error!(error = &err.to_string(), "Failed to find result by mail");
            return Err(RepoError::SomethingWentWrong);
        }

        Ok(())
    }

    async fn update(&self, workspace: UpdateWorkspaceRequest, request_id: Uuid) -> Result<Workspace, RepoError> {
        debug!("Updating workspace by id: {:?}, request_id {}", workspace, request_id);

        let model: workspace::ActiveModel = workspace.into();

        let result = model.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to update user");
            return Err(RepoError::FailedToUpdate);
        }

        Ok(result.unwrap())
    }
}