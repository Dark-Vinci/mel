#[derive(Debug, thiserror::Error)]
pub enum GatewayError {
    #[error("invalid token {0}")]
    InvalidToken(String),
}
