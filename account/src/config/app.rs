#[derive(Debug, Clone)]
pub struct AppConfig {
    pub account_name: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            account_name: "".into(),
        }
    }
}
