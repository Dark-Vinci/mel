use {
    crate::{
        app::{app::App, interfaces::Account},
        models::context::Ctx,
    },
    async_trait::async_trait,
};

#[async_trait]
impl Account for App {
    async fn login_user(&self, _ctx: Ctx, _payload: String) -> String {
        todo!()
    }

    async fn forget_password(&self, _ctx: Ctx, _payload: String) -> String {
        todo!()
    }

    async fn create_user(&self, _ctx: Ctx, _payload: String) -> String {
        todo!()
    }
}
