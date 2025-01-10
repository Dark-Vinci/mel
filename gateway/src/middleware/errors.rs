use {
    axum::http::StatusCode,
    serde::Serialize,
    uuid::Uuid,
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
