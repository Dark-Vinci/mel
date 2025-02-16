use {
    crate::app::{app::App, interface::Reaction},
    async_trait::async_trait,
};

#[async_trait]
impl Reaction for App {}
