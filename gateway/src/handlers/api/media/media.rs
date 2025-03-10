use {
    crate::{
        errors::GatewayError, handlers::handler::AppState,
        middleware::context::Context,
    },
    axum::{
        extract::{Multipart, State},
        routing::post,
        Router,
    },
    sdk::utils::types::FileInfo,
    std::sync::Arc,
};

pub fn router() -> Router {
    Router::new()
        .route("/single", post(upload_file))
        .route("/multiple", post(upload_multiple_files))
}

async fn upload_file(
    mut multipart: Multipart,
    State(state): State<Arc<AppState>>,
    Context(ctx): Context,
) -> Result<FileInfo, GatewayError> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name =
            field.file_name().map(|s| s.to_string()).unwrap_or_default();

        let content_type = field
            .content_type()
            .map(|s| s.to_string())
            .unwrap_or_default();

        let data = field.bytes().await.unwrap();

        let mut file_info = FileInfo::new(file_name, content_type, &data);

        // TODO: Upload to S3 bucket
        let _ = state.app.upload(ctx, &mut file_info).await?;

        return Ok(file_info);
    }

    Err(GatewayError::Generic)
}

async fn upload_multiple_files(
    mut multipart: Multipart,
    State(state): State<Arc<AppState>>,
    Context(ctx): Context,
) -> Result<Vec<FileInfo>, GatewayError> {
    let mut file_uploads = vec![];

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name =
            field.file_name().map(|s| s.to_string()).unwrap_or_default();

        let content_type = field
            .content_type()
            .map(|s| s.to_string())
            .unwrap_or_default();

        let data = field.bytes().await.unwrap();

        let mut file_info = FileInfo::new(file_name, content_type, &data);

        // todo: upload to s3 bucket
        let _ = state.app.upload(ctx.clone(), &mut file_info).await?;

        file_uploads.push(file_info);
    }

    Ok(file_uploads)
}
