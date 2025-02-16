use {
    crate::app::{app::App, interface::Message},
    async_trait::async_trait,
};

#[async_trait]
impl Message for App {}
