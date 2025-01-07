#[derive(Debug, Clone)]
pub struct AppConfig {
    pub service_name: String,
    pub app_name: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            service_name: "".into(),
            app_name: "".into(),
        }
    }
}
