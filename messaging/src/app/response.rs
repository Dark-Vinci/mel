use {
    crate::app::{app::App, interface::Response},
    async_trait::async_trait,
};

#[async_trait]
impl Response for App {}
