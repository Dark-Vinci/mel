use {
    axum::{http::StatusCode, response::IntoResponse},
    serde::Serialize,
};

#[derive(Clone, Debug, Serialize)]
pub struct ApiError {
    pub status_code: StatusCode,
    pub request_id: Uuid,
    pub timestamp: String,
    pub message: String,
    pub generic: String,
}

impl ApiError {
    pub fn new(
        status: StatusCode,
        message: String,
        request_id: Uuid,
        ts: String,
    ) -> Self {
        Self {
            status_code: status,
            message,
            generic: "failure".into(),
            request_id,
            timestamp: ts,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let serialized = serde_json::to_string(&self).unwrap(); // todo: solve the lint error

        (self.status_code, axum::Json(payload)).into_response()
    }
}
