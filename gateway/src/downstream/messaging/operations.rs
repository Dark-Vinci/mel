use {
    crate::downstream::messaging::messaging::{Messaging, MessagingOperations},
    async_trait::async_trait,
};

#[async_trait]
impl MessagingOperations for Messaging {}
