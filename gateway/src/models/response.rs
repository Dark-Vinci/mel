// use {
//     axum::http::StatusCode,
//     serde:: Serialize,
//     uuid::Uuid,
// };


pub struct SuccessResponse {
    data: String,
    message: String,
    status_code: u16,
    request_id: u8,
}

