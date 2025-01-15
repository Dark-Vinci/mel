use thiserror::Error;

#[derive(Debug, Error)]
pub enum GatewayError {
    #[error("invalid token {0}")]
    InvalidToken(String),
}
