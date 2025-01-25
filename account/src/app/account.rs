use std::{future::Future, pin::Pin};
use {
    crate::app::{app::App, interface::Account},
    // anyhow::bail,
    // sdk::errors::GrpcError,
};

impl Account for App {
    fn name<'life0, 'async_trait>(
        &'life0 self,
    ) -> Pin<
        Box<dyn Future<Output = Result<String, String>> + Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
    // async fn name(&self) -> Result<String, GrpcError> {
    //     if let a = &*self.redis {
    //         bail!(GrpcError::Generic)
    //     }

    //     "".into()
    // }
}
