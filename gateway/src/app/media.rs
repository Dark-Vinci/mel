use {
    crate::{
        app::{app::App, interfaces::MediaUploads},
        errors::GatewayError,
        models::context::Ctx,
    },
    async_trait::async_trait,
    sdk::utils::{
        objects::{ContentType, Object, ObjectCreateResponse},
        types::FileInfo,
    },
    tracing::{debug, error},
    uuid::Uuid,
};

#[async_trait]
impl MediaUploads for App {
    #[tracing::instrument(skip(self, payload), name = "App::media::upload")]
    async fn upload(
        &self,
        ctx: Ctx,
        payload: &mut FileInfo,
    ) -> Result<ObjectCreateResponse, GatewayError> {
        // todo: really learn to use the tracing crate
        debug!(message = "Sending media upload request");

        let name = &payload.file_name;
        let key = format!("{}-{}", Uuid::new_v4(), ctx.user_id.unwrap());
        let bucket = self.config.uploads_bucket.clone();

        payload.key = Some(key.clone());

        payload.bucket = Some(bucket.clone());

        // hurray; turbo fish
        let content_type = payload
            .content_type
            .parse::<ContentType>()
            .map_err(|_| GatewayError::Generic)?;

        let result = self
            .object_store
            .upload(Object {
                bucket,
                key,
                name: String::from(name),
                content_type,
                content: Vec::from(&*payload.data),
            })
            .await
            .map_err(|err| {
                error!(
                    display = %err,
                    debug = ?err,
                    "error uploading media object"
                );

                return GatewayError::ObjectStore;
            })?;

        Ok(result)
    }
}
