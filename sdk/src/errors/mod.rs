use thiserror::Error;

pub type AppError = Box<dyn std::error::Error>;

#[derive(Debug, Error)]
pub enum GrpcError {
    #[error("something went wrong")]
    Generic,

    #[error("invalid uuid: {0}")]
    InvalidID(String),
}

#[derive(Debug, Error)]
pub enum RepoError {
    DuplicateKey,

    #[error("invalid model")]
    FailedToInsert,

    #[error("not found")]
    NotFound,

    #[error("something went wrong")]
    SomethingWentWrong,

    FailedToUpdate,
}
