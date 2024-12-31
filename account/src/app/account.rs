use {
    crate::app::{app::App, interface::Account},
    anyhow::bail,
    sdk::errors::GrpcError,
};

impl Account for App {
    async fn name(&self) -> Result<String, GrpcError> {
        if let a = &*self.redis {
            bail!(GrpcError::Generic)
        }

        "".into()
    }
}
