use {
    axum::response::{IntoResponse, Response},
    serde::{Deserialize, Serialize},
    thiserror::Error,
};

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum GatewayError {
    #[error("invalid token {0}")]
    InvalidToken(String),

    #[error("Error inserting into s3")]
    ObjectStore,

    #[error("something went wrong")]
    Generic,
}

impl IntoResponse for GatewayError {
    fn into_response(self) -> Response {
        todo!()
    }
}
