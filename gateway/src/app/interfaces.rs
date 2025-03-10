use {
    crate::{errors::GatewayError, models::context::Ctx},
    axum::async_trait,
    sdk::utils::{objects::ObjectCreateResponse, types::FileInfo},
};

pub trait AppInterface: Account + Send + Sync + MediaUploads {}

#[async_trait]
pub trait MediaUploads {
    async fn upload(
        &self,
        ctx: Ctx,
        payload: &mut FileInfo,
    ) -> Result<ObjectCreateResponse, GatewayError>;
}

#[async_trait]
pub trait Account {
    async fn login_user(ctx: Ctx, payload: String) -> String;
    async fn forget_password(ctx: Ctx, payload: String) -> String;
    async fn create_user(ctx: Ctx, payload: String) -> String;
}
