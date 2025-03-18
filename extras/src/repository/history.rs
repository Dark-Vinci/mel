use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, Condition, DbErr, EntityTrait,
    IntoActiveModel, PaginatorTrait, QuerySelect,
};
use tonic::codegen::tokio_stream::StreamExt;
use tracing::{debug, error};
use uuid::Uuid;
use {crate::connections::db::DB, async_trait::async_trait};
use sdk::errors::RepoError;
use sdk::models::others::extras::CreateHistory;

use sdk::{
    errors::RepoResult,
    models::{
        db::extras::history::{
            ActiveModel, Entity as HistoryEntity, Model as History,
        },
        others::extras::CreateShortUrl,
    },
};
use sdk::models::db::extras::search::Column;
use sdk::models::others::{Paginated, Pagination};

#[async_trait]
pub trait HistoryRepository {
    async fn create(&self, payload: CreateHistory, request_id: Uuid) -> RepoResult<History>;
    async fn get_by_workspace_user_id(&self, workspace_user_id: Uuid, pagination: Pagination, request_id: Uuid) -> RepoResult<History>;
}

pub struct HistoryRepo(DB);

impl HistoryRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl HistoryRepository for HistoryRepo {
    #[tracing::instrument(name = "HistoryRepo::create", skip(self))]
    async fn create(&self, payload: CreateHistory, request_id: Uuid) -> RepoResult<History> {
        debug!(
            request_id = request_id,
            "Got request to create a new history"
        );

        let history: ActiveModel = payload.into();

        let result = history.insert(&self.0.connection).await.map_err(|err| {
            error!(
                display = %err,
                debug = ?err,
                "Failed to create a history table"
            );

            if err == DbErr::RecordNotInserted {
                return RepoError::FailedToInsert;
            }

            return RepoError::SomethingWentWrong;
        })?;

        Ok(result)
    }

    #[tracing::instrument(name = "HistoryRepo::get_by_workspace_user_id", skip(self))]
    async fn get_by_workspace_user_id(&self, workspace_user_id: Uuid, pagination: Pagination, request_id: Uuid) -> RepoResult<History> {
        debug!(
            request_id = request_id,
            "Got a request to fetch paginated users workspace history"
        );

        let (result, count) = tokio::join!(
            HistoryEntity::find()
                .filter(
                    Condition::all()
                        .add(Column::WorkspaceUserId.eq(workspace_user_id))
                )
                .limit(Some(pagination.page_size).into())
                .all(&self.0.connection),

            HistoryEntity::find()
                .filter(
                    Condition::all()
                        .add(Column::WorkspaceUserId.eq(workspace_user_id))
                )
                .count(&self.0.connection)
        );

        let result = result.map_err(|err| {
            error!(
                display = %err,
                debug = ?err,
                "Unable to fetch paginated history"
            );

            RepoError::SomethingWentWrong
        })?;

        let count = count.map_err(|err| {
            error!(
                display = %err,
                debug = ?err,
                "Unable to count paginated history"
            );

            RepoError::SomethingWentWrong
        })?;

        let paginated = Paginated::new(
            result,
            count / pagination.size,
            pagination.page_number + 1,
            pagination.page_size,
            count,
        );

        Ok(paginated)
    }
}
