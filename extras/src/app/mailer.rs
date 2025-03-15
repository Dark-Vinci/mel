use {
    crate::app::{app::App, interface::Mailer},
    async_trait::async_trait,
};

#[async_trait]
impl Mailer for App {}
