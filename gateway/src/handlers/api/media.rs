use std::fs::File;
use std::io::Write;
use axum::extract::{Multipart, State};
use axum::Router;
use axum::routing::post;
use crate::handlers::handler::AppState;
use crate::models::context::CTX;
use crate::models::error_response::ApiError;

pub fn router() -> Router {
    Router::new()
        .route("/single", post(upload_file))
        .route("/multiple", post(upload_multiple_files))
}

struct FileInfo<'a> {
    file_path: &'a str,
    file_name: &'a str,
    content_type: &'a str,
    bucket: Option<&'a str>,
    key: Option<&'a str>,
}

async fn upload_file(mut multipart: Multipart, State(state): State<AppState>) -> Result<FileInfo, String> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap(); // Read file content
        let file_name = field.file_name().unwrap();
        let content_type = field.content_type().unwrap_or_default();

        // Save the uploaded file
        let file_path = format!("uploads/{}", file_name);
        let mut file = File::create(&file_path).unwrap();
        file.write_all(&data).unwrap();

        println!("Uploaded: {}", file_path);

        let file_info = FileInfo {
            file_path: &file_path,
            file_name,
            content_type,
            bucket: None,
            key: None,
        };

        // todo: upload to s3 bucket
        let ctx = CTX::new();

        let response = state.app
            .upload(ctx, ())
            .await
            .map_err(|_err| "generic_error".to_string())?;

        return Ok(file_info)
    }

    Err("File upload failed.".to_string())
}

async fn upload_multiple_files(mut multipart: Multipart) -> Result<Vec<FileInfo>, String> {
    let mut file_uploads = vec![];

    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(_file_name) = field.file_name().map(|f| f.to_string()) {
            let content_type = field.content_type().unwrap_or_default();
            let file_name = field.file_name().unwrap_or_default();
            let data = field.bytes().await.unwrap();

            let file_path = &format!("uploads/{}", file_name);
            let mut file = File::create(&file_path).unwrap();
            file.write_all(&data).unwrap();

            let file_info = FileInfo{
                file_path,
                file_name,
                content_type,
                bucket: None,
                key: None,
            };

            file_uploads.push(file_info);
        }
    }

    // todo: upload to object storage

    Ok(file_uploads)
}
