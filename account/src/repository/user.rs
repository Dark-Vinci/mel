use {
    crate::connections::db::DB,
    async_trait::async_trait,
    chrono::{Local, Utc},
    sdk::{
        errors::{RepoError, RepoResult},
        models::{
            db::account::user::{self, Entity as User, Model},
            others::auth::create::{CreateUserRequest, UpdateUserRequest},
        },
    },
    sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait,
        IntoActiveModel, QueryFilter,
    },
    tracing::{debug, error, Level},
    uuid::Uuid,
};

#[async_trait]
pub trait UserRepository {
    async fn create(
        &self,
        user: CreateUserRequest,
        request_id: Uuid,
    ) -> RepoResult<Model>;

    async fn get_by_id(&self, id: Uuid, request_id: Uuid) -> RepoResult<Model>;

    async fn get_by_email(
        &self,
        request_id: Uuid,
        mail: String,
    ) -> RepoResult<Model>;

    async fn soft_delete(
        &self,
        request_id: Uuid,
        id: Uuid,
    ) -> Result<(), RepoError>;

    async fn update(
        &self,
        request_id: Uuid,
        user: UpdateUserRequest,
    ) -> RepoResult<Model>;
}

pub struct UserRepo(DB);

impl UserRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl UserRepository for UserRepo {
    #[tracing::instrument(name = "UserRepository::create", skip(self))]
    async fn create(
        &self,
        user: CreateUserRequest,
        request_id: Uuid,
    ) -> RepoResult<Model> {
        debug!("Creating user: {:?}, with request id: {}", user, request_id);

        let a: user::ActiveModel = user.into();

        let result = a.insert(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = result {
            error!(error = &err.to_string(), "Failed to insert user record");

            if err.to_string().contains("duplicate key") {
                return Err(RepoError::DuplicateKey);
            }

            return Err(RepoError::FailedToInsert);
        }

        Ok(result.unwrap())
    }

    #[tracing::instrument(name = "UserRepository::get_by_id", skip(self))]
    async fn get_by_id(&self, id: Uuid, request_id: Uuid) -> RepoResult<Model> {
        debug!("Getting user by id: {}, with request id: {}", id, request_id);

        let result = User::find_by_id(id).one(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to find user by id");
            return Err(RepoError::SomethingWentWrong);
        }

        let user = result.unwrap();

        if user.is_none() {
            error!("User not found");
            return Err(RepoError::NotFound);
        }

        Ok(user.unwrap())
    }

    #[tracing::instrument(name = "UserRepository::get_by_email", skip(self))]
    async fn get_by_email(
        &self,
        request_id: Uuid,
        mail: String,
    ) -> RepoResult<Model> {
        debug!(
            "Getting user by mail: {}, with request id: {}",
            mail, request_id
        );

        let result = User::find()
            .filter(user::Column::Email.eq(mail))
            .one(&self.0.connection)
            .await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to find user by mail");
            return Err(RepoError::SomethingWentWrong);
        }

        let user = result.unwrap();

        if user.is_none() {
            error!("User not found");
            return Err(RepoError::NotFound);
        }

        Ok(user.unwrap())
    }

    #[tracing::instrument(name = "UserRepository::soft_delete", skip(self))]
    async fn soft_delete(&self, request_id: Uuid, id: Uuid) -> RepoResult<()> {
        debug!("Deleting user by id: {}, request_id {}", id, request_id);

        let mut user =
            self.get_by_id(request_id, id).await?.into_active_model();

        user.deleted_at = Set(Some(Utc::now()));

        let res = user.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &res {
            error!(error = &err.to_string(), "Failed to find user by mail");
            return Err(RepoError::SomethingWentWrong);
        }

        Ok(())
    }

    #[tracing::instrument(name = "UserRepository::update", skip(self))]
    async fn update(
        &self,
        request_id: Uuid,
        user: UpdateUserRequest,
    ) -> RepoResult<Model> {
        debug!("Updating user by id: {:?}, request_id {}", user, request_id);

        let model: user::ActiveModel = user.into();

        let result = model.update(&self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to update user");
            return Err(RepoError::FailedToUpdate);
        }

        Ok(result.unwrap())
    }
}
