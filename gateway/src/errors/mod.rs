use thiserror::Error;

#[derive(Debug, Error)]
pub enum GatewayError {
    #[error("invalid token {0}")]
    InvalidToken(String),

    #[error("Error inserting into s3")]
    ObjectStore,

    #[error("something went wrong")]
    Generic,
}
