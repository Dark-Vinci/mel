use thiserror::Error;

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
