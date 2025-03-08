use axum::async_trait;
use uuid::Uuid;
use sdk::utils::objects::{ContentType, Object};
use crate::app::app::App;
use crate::app::interfaces::MediaUploads;
use crate::models::context::CTX;

#[async_trait]
impl MediaUploads for App {
    #[tracing::instrument(skip(self), name="App::media::upload")]
    async fn upload(&self, ctx: CTX, payload: String) -> Result<(), ()> {
        let key = format!("{}-{}", Uuid::new_v4(), ctx.get_user_id().unwrap()ctx.get_user_id().unwrap());
        let res = self.object_store
            .upload(Object {
                bucket: "".to_string(),
                key,
                name: "".to_string(),
                content_type: ContentType::PDF,
                content: vec![],
            })
        .await?;

        // todo: work more here

        Ok(())
    }
}