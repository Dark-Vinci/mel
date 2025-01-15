use axum::Json;
use serde_json::json;
use {
    axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    },
    sdk::errors::AppError,
    serde::Serialize,
    uuid::Uuid,
};

#[derive(Serialize, Clone)]
pub struct ApiError {
    status_code: u16,
    request_id: Uuid,
    message: String,
    error_message: String,
}

impl ApiError {
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

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let sts = self.status_code;
        let payload = json!({
            "message": self.message,
            "status_code": self.status_code,
            "request_id": self.request_id.to_string(),
        });

        (StatusCode::from_u16(sts).unwrap(), Json(payload)).into_response()
    }
}
