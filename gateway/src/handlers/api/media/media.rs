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
    sdk::utils::types::{FileInfo, VecFile},
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/single", post(upload_file))
        .route("/multiple", post(upload_multiple_files))
        .with_state(state)
}

#[axum_macros::debug_handler]
async fn upload_file(
    Context(ctx): Context,
    State(state): State<AppState>,
    mut multipart: Multipart,
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

#[axum_macros::debug_handler]
async fn upload_multiple_files(
    Context(ctx): Context,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<VecFile, GatewayError> {
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

        let _ = state
            .app
            .upload(ctx.clone(), &mut file_info)
            .await
            .map_err(|_e| {
                return GatewayError::Generic;
            })?;

        file_uploads.push(file_info);
    }

    let result = VecFile::new(file_uploads);

    Ok(result)
}
