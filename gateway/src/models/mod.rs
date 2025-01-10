use {
    crate::errors::GatewayError,
    axum::http,
    serde::{Deserialize, Serialize},
};

#[derive(Debug)]
pub struct LoginStruct {
    pub username: String,
    pub password: String,
}

pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub user_id: String,
}

#[derive(Debug, Clone)]
struct WsRequest<T: Clone> {
    pub action: String,
    pub payload: T,
    pub token: String,
}

#[derive(Debug, Clone)]
struct Message<'a> {
    pub content: String,
    pub to: Type,
    pub files: Vec<&'a str>,
}

#[derive(Debug, Clone)]
enum Type {
    // user id
    Private(String),
    // channel id
    Channel(String),
}

struct WsResponse {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T: Serialize> {
    data: Option<T>,
    message: String,
    status_code: http::StatusCode,
    request_id: String,
    timestamp: std::time::SystemTime,
}

impl<T: Serialize> Response<T> {
    fn new(
        data: Option<T>,
        stat_code: http::StatusCode,
        request_id: uuid::Uuid,
        message: String,
    ) -> Self<T> {
        if data.is_none() {
            return Self {
                data: None,
                message,
                status_code: stat_code,
                request_id: request_id.into(),
                timestamp: std::time::SystemTime::now(),
            };
        }

        Self {
            data,
            message,
            status_code: stat_code,
            request_id: request_id.to_string(),
            timestamp: std::time::SystemTime::now(),
        }
    }
}
