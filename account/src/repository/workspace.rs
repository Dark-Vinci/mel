use {
    crate::connections::db::DB,
    chrono::Utc,
    sdk::{
        errors::RepoError,
        models::{
            db::account::{
                user,
                workspace::{
                    self, Entity as WorkspaceEntity, Model as Workspace,
                },
            },
            others::auth::workspace::{
                CreateWorkspaceRequest, UpdateWorkspaceRequest,
            },
        },
    },
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait, IntoActiveModel,
    },
    tracing::{debug, error},
    uuid::Uuid,
};
use sdk::errors::RepoResult;

#[async_trait::async_trait]
pub trait WorkspaceRepository {
    async fn create(
        &self,
        result: CreateWorkspaceRequest,
        request_id: Uuid,
    ) -> RepoResult<Workspace>;

    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<Workspace>;

    async fn delete(&self, id: Uuid, request_id: Uuid)
        -> Result<(), RepoError>;

    async fn update(
        &self,
        result: UpdateWorkspaceRequest,
        request_id: Uuid,
    ) -> RepoResult<Workspace>;
}

pub struct WorkspaceRepo(DB);

impl WorkspaceRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait::async_trait]
impl WorkspaceRepository for WorkspaceRepo {
    #[tracing::instrument(name = "WorkspaceRepository::create", skip(self))]
    async fn create(
        &self,
        workspace: CreateWorkspaceRequest,
        request_id: Uuid,
    ) -> RepoResult<Workspace> {
        debug!(
            "Creating workspace: {:?}, with request id: {}",
            workspace, request_id
        );

        let work_active: workspace::ActiveModel = workspace.into();

        let result = work_active.insert(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = result {
            error!(error = &err.to_string(), "Failed to insert result record");

            if err.to_string().contains("duplicate key") {
                return Err(RepoError::DuplicateKey);
            }

            return Err(RepoError::FailedToInsert);
        }

        Ok(result.unwrap())
    }

    #[tracing::instrument(name = "WorkspaceRepository::get_by_id", skip(self))]
    async fn get_by_id(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> RepoResult<Workspace> {
        debug!(
            "Getting workspace by id: {}, with request id: {}",
            id, request_id
        );

        let result = WorkspaceEntity::find_by_id(id)
            .one(&self.0.connection)
            .await;

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

    #[tracing::instrument(name = "WorkspaceRepository::delete", skip(self))]
    async fn delete(
        &self,
        id: Uuid,
        request_id: Uuid,
    ) -> Result<(), RepoError> {
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

    #[tracing::instrument(name = "WorkspaceRepository::update", skip(self))]
    async fn update(
        &self,
        workspace: UpdateWorkspaceRequest,
        request_id: Uuid,
    ) -> RepoResult<Workspace> {
        debug!(
            "Updating workspace by id: {:?}, request_id {}",
            workspace, request_id
        );

        let model: workspace::ActiveModel = workspace.into();

        let result = model.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to update user");
            return Err(RepoError::FailedToUpdate);
        }

        Ok(result.unwrap())
    }
}
