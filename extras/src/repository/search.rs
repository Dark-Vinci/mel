use {
    crate::connections::db::DB,
    async_trait::async_trait,
    sdk::{
        errors::RepoError,
        models::{
            db::extras::search::{
                ActiveModel, Column, Entity as SearchEntity, Model as Search,
            },
            others::{extras::CreateSearch, Paginated, Pagination},
        },
    },
    sea_orm::{
        ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait,
        QueryFilter, QuerySelect,
    },
    tracing::{debug, error},
    uuid::Uuid,
};

#[async_trait]
pub trait SearchRepository {
    async fn create(
        &self,
        payload: CreateSearch,
        request_id: Uuid,
    ) -> Result<Search, RepoError>;

    async fn get_by_workspace_user_id(
        &self,
        workspace_user_id: Uuid,
        page: Pagination,
        request_id: Uuid,
    ) -> Result<Paginated<Vec<Search>>, RepoError>;
}

pub struct SearchRepo(DB);

impl SearchRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl SearchRepository for SearchRepo {
    #[tracing::instrument(name = "SearchRepo::create", skip(self))]
    async fn create(
        &self,
        payload: CreateSearch,
        request_id: Uuid,
    ) -> Result<Search, RepoError> {
        debug!(
            request_id = %request_id,
            "Got request to create user search"
        );

        let search: ActiveModel = payload.into();

        let result =
            search.insert(&self.0.connection).await.map_err(|err| {
                error!(
                    display = %err,
                    debug = ?err,
                    "Could not create search"
                );

                if err == DbErr::RecordNotInserted {
                    return RepoError::FailedToInsert;
                }

                return RepoError::FailedToInsert;
            })?;

        Ok(result)
    }

    #[tracing::instrument(
        name = "SearchRepo::get_by_workspace_user_id",
        skip(self)
    )]
    async fn get_by_workspace_user_id(
        &self,
        workspace_user_id: Uuid,
        pagination: Pagination,
        request_id: Uuid,
    ) -> Result<Paginated<Vec<Search>>, RepoError> {
        debug!(
            request_id = %request_id,
            "Got request to get search by workspace_user_id"
        );

        let (result, count): (Result<Vec<Search>, DbErr>, Result<u64, DbErr>) = tokio::join!(
            SearchEntity::find()
                .filter(Column::WorkspaceUserId.eq(workspace_user_id))
                .limit(Some(pagination.page_size))
                .all(&self.0.connection),
            SearchEntity::find()
                .filter(Column::WorkspaceUserId.eq(workspace_user_id))
                .count(&self.0.connection)
        );

        let result = result.map_err(|err| {
            error!(
                display = %err,
                debug = ?err,
                "Unable to fetch paginated search"
            );

            RepoError::SomethingWentWrong
        })?;

        let count = count.map_err(|err| {
            error!(
                display = %err,
                debug = ?err,
                "Unable to count paginated search"
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
