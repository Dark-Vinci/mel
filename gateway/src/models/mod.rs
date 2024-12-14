use axum::http;
use serde::{Deserialize, Serialize};
use crate::errors::{GatewayError};

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

struct WsRequest<T> {
    pub action: String,
    pub payload: T,
    pub token: String,
}

struct WsResponse {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T: Clone + Copy + serde::ser::Serialize + serde::de::DeserializeOwned> {
    data: Option<T>,
    error: Option<GatewayError>,
    message: String,
    status_code: http::StatusCode,
    request_id: String,
    timestamp: std::time::SystemTime,
}

impl<T> Response<T> {
    fn new(
        data: Option<T>,
        stat_code: http::StatusCode,
        request_id: uuid::Uuid,
        message: String,
        error: Option<GatewayError>,
    ) -> Self<T> {
        if data.is_none() {
            return Self {
                data: None,
                error,
                message,
                status_code: stat_code,
                request_id: request_id.to_string(),
                timestamp: std::time::SystemTime::now(),
            }
        }

        Self {
            data,
            error: None,
            message,
            status_code: stat_code,
            request_id: request_id.to_string(),
            timestamp: std::time::SystemTime::now(),
        }
    }
}
