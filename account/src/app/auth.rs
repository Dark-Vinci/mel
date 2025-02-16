use {
    crate::app::{app::App, interface::Auth},
    async_trait::async_trait,
};

#[async_trait]
impl Auth for App {}
