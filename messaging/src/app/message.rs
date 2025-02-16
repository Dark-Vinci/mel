use async_trait::async_trait;
use crate::app::app::App;
use crate::app::interface::Message;

#[async_trait]
impl Message for App {}