use async_trait::async_trait;
use crate::downstream::messaging::messaging::{Messaging, MessagingOperations};

#[async_trait]
impl MessagingOperations for Messaging {}