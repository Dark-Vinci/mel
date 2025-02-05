use {
    crate::app::{app::App, interface::Auth},
    async_trait::async_trait,
    sdk::{
        errors::GrpcError,
        models::{
            db::account::channel::Model as Channel,
            others::auth::channel::{CreateChannel, CreateChannelUser},
        },
    },
    tracing::{debug, error},
    uuid::Uuid,
};

#[async_trait]
impl Auth for App {}
