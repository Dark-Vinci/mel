use sea_orm::{DbErr, EntityTrait};
use {
    crate::connections::db::DB,
    async_trait::async_trait,
    sdk::models::{
        db::{
            auth,
            auth::{user::Entity as User, user::Model, user},
        },
        others::auth::create::CreateUserRequest,
    },
    sea_orm::{ActiveModelTrait},
    tracing::{debug, error, Level},
    uuid::Uuid,
};
use sdk::errors::RepoError;

#[async_trait]
pub trait UserRepository {
    async fn create(
        &self,
        user: CreateUserRequest,
        request_id: Uuid,
    ) -> Model;

    async fn get_by_id(&self, id: Uuid, request_id: Uuid) -> Result<Model, RepoError>;
    async fn get_by_email(&self, request_id: Uuid, mail: String) -> Result<Model, RepoError>
}

pub struct UserRepo<'a>(&'a DB);

impl UserRepo {
    pub fn new(db: &DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl UserRepository for UserRepo {
    #[tracing::instrument(
    name = "UserRepository::create",
    skip(self),
    error=(level = Level::ERROR),
    level=Level::DEBUG,
    )]
    async fn create(
        &self,
        user: CreateUserRequest,
        request_id: Uuid,
    ) -> Result<Model, RepoError> {
        debug!("Creating user: {:?}, with request id: {}", user, request_id);

        let a: user::ActiveModel = user.into();

        let result = a.insert(&*self.0.connection).await;

        if let Err(DbErr::Exec(err)) = result {
            error!(error = &err.to_string(), "Failed to insert user record");

            if &err.to_string().contains("duplicate key") {
                return Err(RepoError::DuplicateKey);
            }

            return Err(RepoError::FailedToInsert);
        }


        Ok(result.unwrap())
    }

    #[tracing::instrument(
    name = "UserRepository::get_by_id",
    skip(self),
    error=(level = Level::ERROR),
    level=Level::DEBUG,
    )]
    async fn get_by_id(&self, id: Uuid, request_id: Uuid) -> Result<Model, RepoError> {
        debug!("Getting user by id: {}, with request id: {}", id, request_id);

        let result  = User::find_by_id(id).one(&*self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to find user by id");
            return Err(RepoError::SomethingWentWrong)
        }

        let user = result.unwrap();

        if &user.is_none() {
            error!("User not found");
            return Err(RepoError::NotFound);
        }

        Ok(user.unwrap())
    }

    #[tracing::instrument(
    name = "UserRepository::get_by_email",
    skip(self),
    error=(level = Level::ERROR),
    level=Level::DEBUG,
    )]
    async fn get_by_email(&self, request_id: Uuid, mail: String) -> Result<Model, RepoError> {
        debug!("Getting user by mail: {}, with request id: {}", mail, request_id);

        let result  = User::find().filter(user::Column::Email.eq(mail)).one(&*self.0.connection).await;

        if let Err(DbErr::Exec(err)) = &result {
            error!(error = &err.to_string(), "Failed to find user by mail");
            return Err(RepoError::SomethingWentWrong)
        }

        let user = result.unwrap();

        if &user.is_none() {
            error!("User not found");
            return Err(RepoError::NotFound);
        }

        Ok(user.unwrap())
    }
}
