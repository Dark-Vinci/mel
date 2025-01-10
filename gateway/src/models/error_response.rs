use {
    axum::http::StatusCode,
    sdk::errors::AppError,
    serde:: Serialize,
    uuid::Uuid,
    axum::{response::IntoResponse, Json},
};

#[derive(Serialize, Clone)]
pub struct ErrorResponse {
    status_code: u16,
    request_id: Uuid,
    message: String,
    error_message: String,
}

impl ErrorResponse {
    pub fn new(
        status_code: StatusCode,
        request_id: Uuid,
        message: String,
        error: AppError,
    ) -> Self {
        Self {
            request_id,
            message,
            status_code: status_code.as_u16(),
            error_message: error.to_string(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let serialized = Json(self.clone());
        let status_code = StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (status_code, serialized).into_response()
    }
}