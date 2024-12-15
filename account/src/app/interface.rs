pub trait Auth {}

#[async_trait::async_trait]
pub trait Account {
    async fn name(&self) -> &str;
}
pub trait Settings {}

pub trait AccountInterface: Auth + Account + Settings {}
