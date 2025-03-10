use {
    crate::{
        app::{app::App, interfaces::MediaUploads},
        errors::GatewayError,
        models::context::CTX,
    },
    axum::async_trait,
    sdk::utils::{
        objects::{ContentType, Object, ObjectCreateResponse},
        types::FileInfo,
    },
    std::str::FromStr,
    tracing::{debug, error},
    uuid::Uuid,
};

#[async_trait]
impl MediaUploads for App {
    #[tracing::instrument(skip(self, payload.data), name = "App::media::upload")]
    async fn upload(
        &self,
        ctx: CTX,
        payload: &mut FileInfo,
    ) -> Result<ObjectCreateResponse, GatewayError> {
        // todo: really learn to use the tracing crate
        debug!(message = "Sending media upload request", ctx);

        payload.key =
            Some(&format!("{}-{}", Uuid::new_v4(), ctx.get_user_id().unwrap()));

        payload.bucket = Some(&self.config.uploads_bucket);

        let content_type = ContentType::from_str(&payload.content_type)
            .map_err(|_| GatewayError::Generic)?;

        let result = self
            .object_store
            .upload(Object {
                bucket: String::from(payload.bucket),
                key: String::from(payload.key),
                name: String::from(payload.file_name),
                content_type,
                content: Vec::from(*payload.data),
            })
            .await
            .map_err(|err| {
                error!("Failed to upload object {}", err);
                return GatewayError::ObjectStore;
            })?;

        Ok(result)
    }
}
