use {
    axum::http::StatusCode,
    sdk::errors::AppError,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Deserialize, Serialize)]
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

#[derive(Serialize, Debug)]
pub struct SuccessResponse<T: Serialize> {
    data: T,
    message: String,
    status_code: u16,
    request_id: Uuid,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn new(
        data: T,
        status_code: StatusCode,
        request_id: Uuid,
        message: String,
    ) -> Self {
        Self {
            data,
            message,
            request_id,
            status_code: status_code.as_u16(),
        }
    }
}
