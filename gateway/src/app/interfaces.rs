use {crate::models::context::CTX, axum::async_trait};

pub trait AppInterface: Account + Send + Sync + MediaUploads {}


#[async_trait]
pub trait MediaUploads{
    async fn upload(&self, ctx: CTX, payload: ()) -> Result<(), ()>;
}

#[async_trait]
pub trait Account {
    async fn login_user(ctx: CTX, payload: String) -> String;
    async fn forget_password(ctx: CTX, payload: String) -> String;
    async fn create_user(ctx: CTX, payload: String) -> String;
}
