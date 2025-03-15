use async_trait::async_trait;
use crate::downstream::account::account::{Account, AccountOperations};

#[async_trait]
impl AccountOperations for Account {}