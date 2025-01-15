use {
    axum::{http::StatusCode, response::IntoResponse, Json},
    serde::Serialize,
    uuid::Uuid,
};

#[derive(Serialize, Debug, Clone)]
pub struct SuccessResponse<T: Serialize + Clone> {
    data: T,
    message: String,
    status_code: u16,
    request_id: Uuid,
}

impl<T: Serialize + Clone> SuccessResponse<T> {
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

impl<T: Serialize + Clone> IntoResponse for SuccessResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let serialized = Json(self.clone());
        let status_code = StatusCode::from_u16(self.status_code)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (status_code, serialized).into_response()
    }
}
