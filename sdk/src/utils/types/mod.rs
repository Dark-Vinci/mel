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

#[derive(Serialize, Deserialize)]
pub struct VecFile{
    pub value: Vec<FileInfo>,
}

impl VecFile {
    pub fn new(value: Vec<FileInfo>) -> Self {
        Self { value }
    }
}

impl IntoResponse for VecFile {
    fn into_response(self) -> Response {
        todo!()
    }
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