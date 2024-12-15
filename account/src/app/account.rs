use crate::app::{app::App, interface::Account};

impl Account for App {
    async fn name(&self) -> &str {
        "".into()
    }
}
