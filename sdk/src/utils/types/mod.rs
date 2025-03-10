use axum::body::Bytes;

pub struct FileInfo<'a> {
    pub file_name: &'a str,
    pub content_type: &'a str,
    pub bucket: Option<&'a str>,
    pub key: Option<&'a str>,
    pub data: Bytes
}

impl<'a> FileInfo<'a> {
    pub fn new(file_name: &'a str, content_type: &'a str, data: &Bytes) -> Self {
        Self {
            file_name,
            content_type,
            bucket,
            key,
            data: Bytes::from(data),
        }
    }
}