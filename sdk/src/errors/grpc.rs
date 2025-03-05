use thiserror::Error;

#[derive(Debug, Error)]
pub enum GrpcError {
    #[error("something went wrong")]
    Generic,

    #[error("{0} already exist")]
    AlreadyExists(String),

    #[error("{0} does not exist")]
    NotFound(String),

    #[error("invalid uuid: {0}")]
    InvalidID(String),
}
