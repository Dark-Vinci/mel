use thiserror::Error;

pub type AppError = Box<dyn std::error::Error>;

#[derive(Debug, Error)]
pub enum GrpcError {
    #[error("something went wrong")]
    Generic,

    #[error("{0} already exist")]
    AlreadyExists(String),

    #[error("invalid uuid: {0}")]
    InvalidID(String),
}

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("DB connection error")]
    DB(String),
}

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Duplicate key")]
    DuplicateKey,

    #[error("invalid model")]
    FailedToInsert,

    #[error("not found")]
    NotFound,

    #[error("something went wrong")]
    SomethingWentWrong,

    #[error("Fail to update")]
    FailedToUpdate,
}
