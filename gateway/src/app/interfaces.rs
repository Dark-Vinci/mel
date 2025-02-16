use axum::async_trait;
use crate::models::context::CTX;

pub trait AppInterface: Account + Send + Sync {}

#[async_trait]
pub trait Account {
    async fn login_user(ctx: CTX, payload: String) -> String;
}
