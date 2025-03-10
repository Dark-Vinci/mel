// use axum::body::Bytes;

use axum::body::Bytes;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    pub file_name: String,
    pub content_type: String,
    pub bucket: Option<String>,
    pub key: Option<String>,
    pub data: Vec<u8>
}

impl IntoResponse for FileInfo {
    fn into_response(self) -> Response {
        todo!()
    }
}

impl FileInfo {
    pub fn new(file_name: String, content_type: String, _data: &Bytes) -> Self {
        Self {
            file_name,
            content_type,
            bucket: None,
            key: None,
            data: vec![],
        }
    }
}