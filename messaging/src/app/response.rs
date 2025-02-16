use async_trait::async_trait;
use crate::app::{app::App};
use crate::app::interface::Response;

#[async_trait]
impl Response for App {}
